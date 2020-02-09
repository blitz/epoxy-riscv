let Prelude =
        env:DHALL_PRELUDE sha256:4aa8581954f7734d09b7b21fddbf5d8df901a44b54b4ef26ea71db92de0b1a12
      ? https://prelude.dhall-lang.org/v13.0.0/package.dhall

let concat = Prelude.List.concat

let empty = Prelude.List.empty

let map = Prelude.List.map

let GenericObject = λ(ref : Type) → { gid : Text, references : List ref }

let Object
    : Type
    = ∀(Object : Type) → ∀(MakeObject : GenericObject Object → Object) → Object

let example
    : Object
    =   λ(Object : Type)
      → λ(MakeObject : GenericObject Object → Object)
      → MakeObject
          { gid = "root"
          , references =
            [ MakeObject { gid = "leaf", references = empty Object }
            , MakeObject
                { gid = "middle"
                , references =
                  [ MakeObject { gid = "leaf", references = empty Object } ]
                }
            ]
          }

let FlattenType
    : Type
    = { indirect : List (GenericObject Text), direct : GenericObject Text }

let squash = λ(f : FlattenType) → f.indirect # [ f.direct ]

let allDirectRef = map FlattenType Text (λ(f : FlattenType) → f.direct.gid)

let allChildren =
        λ(fl : List FlattenType)
      → concat
          (GenericObject Text)
          (map FlattenType (List (GenericObject Text)) squash fl)

let flatten
    : Object → FlattenType
    =   λ(x : Object)
      → x
          FlattenType
          (   λ(p : { references : List FlattenType, gid : Text })
            → { direct =
                  { gid = p.gid
                  , references = allDirectRef p.references : List Text
                  }
              , indirect = allChildren p.references : List (GenericObject Text)
              }
          )

let PrefixType = ∀(prefix : Text) → Object

let prefixNames
    : Object → PrefixType
    =   λ(x : Object)
      → x
          PrefixType
          (   λ(p : { references : List PrefixType, gid : Text })
            → λ(prefix : Text)
            → λ(IObject : Type)
            → λ(MakeObject : GenericObject IObject → IObject)
            → let newPrefix = prefix ++ p.gid

              in  MakeObject
                    { gid = newPrefix
                    , references =
                        map
                          PrefixType
                          IObject
                          (   λ(child : PrefixType)
                            → child (newPrefix ++ "_") IObject MakeObject
                          )
                          p.references
                    }
          )

in  squash (flatten (prefixNames example ""))
