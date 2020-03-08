#pragma once

// Some simple iterator transformation code.
//
// Someone who actually knows how to do metaprogramming in C++ should
// probably redo them...

namespace esl {

  template <typename predicate_type,
	    typename range_type>
  class filter_range {
    predicate_type predicate;
    range_type &range;

    using range_it = decltype(range.begin());

  public:

    class filtered_it {
      predicate_type predicate;

      range_it source_it;
      range_it end_it;

    public:

      auto operator*()
      {
	return *source_it;
      }

      filtered_it &operator++()
      {
	do {
	  ++source_it;
	} while (source_it != end_it and not predicate(*source_it));

	return *this;
      }

      bool operator==(filtered_it const &other) const
      {
	return source_it == other.source_it;
      }

      bool operator!=(filtered_it const &other) const
      {
	return source_it != other.source_it;
      }

      filtered_it(predicate_type const &predicate_, range_it const &source_it_, range_it const &end_it_)
	: predicate{predicate_}, source_it{source_it_}, end_it{end_it_}
      {
	while (source_it != end_it and not predicate(*source_it)) {
	  ++source_it;
	}
      }
    };

    filtered_it begin()
    {
      return {predicate, range.begin(), range.end()};
    }

    filtered_it end()
    {
      return {predicate, range.end(), range.end()};
    }

    filter_range(range_type &range_, predicate_type const &predicate_)
      : predicate{predicate_}, range{range_}
    {}
  };

  template <typename transform_type,
	    typename range_type>
  class transform_range {
    transform_type transform;
    range_type &range;

    using range_it = decltype(range.begin());

  public:

    class transformed_it {
      transform_type transform;

      range_it source_it;

    public:

      auto operator*()
      {
	return transform(*source_it);
      }

      transformed_it &operator++()
      {
	++source_it;
	return *this;
      }

      bool operator==(transformed_it const &other) const
      {
	return source_it == other.source_it;
      }

      bool operator!=(transformed_it const &other) const
      {
	return source_it != other.source_it;
      }

      transformed_it(transform_type const &transform_, range_it const &source_it_)
	: transform{transform_}, source_it{source_it_}
      {}
    };

    transformed_it begin()
    {
      return {transform, range.begin()};
    }

    transformed_it end()
    {
      return {transform, range.end()};
    }

    transform_range(range_type &range_, transform_type const &transform_)
      : transform{transform_}, range{range_}
    {}
  };
};
