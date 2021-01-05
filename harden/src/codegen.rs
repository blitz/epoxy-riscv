//! Generate process configuration code from resource descriptions.

use std::fmt::{Display, Formatter};
use std::str::FromStr;
use itertools::Itertools;

use crate::framebuffer;
use crate::runtypes;

/// The supported programming languages for code generation output.
#[derive(Copy, Clone)]
pub enum Language {
    /// C++
    CPP,
}

static LANGUAGE_NAMES: [(&'static str, Language); 1] = [("c++", Language::CPP)];

/// The error that is returned for failure to parse a string into `Language`.
#[derive(Debug)]
pub struct LanguageParseError {
    /// The string that was not recognized as a language.
    unrecognized: String,
}

impl Display for LanguageParseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "Unrecognized language '{}', must be one of: {}",
            self.unrecognized,
            LANGUAGE_NAMES
                .iter()
                .map(|&(k, _)| k)
                .collect::<Vec<&'static str>>()
                .join(" ")
        )
    }
}

impl std::error::Error for LanguageParseError {}

impl FromStr for Language {
    type Err = LanguageParseError;

    fn from_str(s: &str) -> std::result::Result<Self, <Self as std::str::FromStr>::Err> {
        Ok(LANGUAGE_NAMES
            .iter()
            .find(|&(k, _)| k.eq_ignore_ascii_case(s))
            .ok_or_else(|| LanguageParseError {
                unrecognized: s.to_owned(),
            })?
            .1)
    }
}

fn generate_cpp_res(name: &str, resource: &runtypes::MemoryResource) -> String {
    match &resource.meta {
        runtypes::ResourceMetaInfo::Framebuffer { format } => {
            if format.pixel != framebuffer::PixelFormat::R5G6B5 {
                todo!("Implement different pixel formats");
            };

            if format.stride % 2 != 0 {
                todo!("How do we deal with strides that are not multiples of the pixel size?");
            }

            format!("
static inline uint16_t volatile (&{}_pixels)[{}][{}] {{*reinterpret_cast<uint16_t volatile (*)[{}][{}]>({:#x})}};
static inline size_t {}_width {{{}}};
", name, format.height, format.stride / 2, format.height, format.stride / 2, resource.region.virt_start, name, format.width)
        }
    }
}

pub fn generate(language: Language, process: &runtypes::Process) -> String {
    match language {
        Language::CPP => format!(
            "// Automatically generated. Do not touch.

#pragma once

#include <cstddef>
#include <cstdint>
{}",
            process
                .resources
                .iter()
                .map(|(name, res)| generate_cpp_res(name, res))
                .join("\n")
        ),
    }
}