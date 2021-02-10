//! Generate process configuration code from resource descriptions.

use itertools::Itertools;
use std::fmt::{Display, Formatter};
use std::str::FromStr;

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

fn generate_cpp_res(name: &str, resource: &runtypes::Resource) -> String {
    match &resource.meta {
        runtypes::ResourceMetaInfo::SifivePlic { ndev } => format!(
            "
constexpr uint16_t {}_ndev {{{}}};
inline uint32_t volatile * const {}_reg {{reinterpret_cast<uint32_t volatile *>({:#x}ul)}};
",
            name,
            ndev,
            name,
            resource
                .opt_region
                .as_ref()
                .expect("PLIC without memory region")
                .virt_start
        ),
        runtypes::ResourceMetaInfo::Framebuffer { format } => {
            if format.pixel != framebuffer::PixelFormat::R5G6B5 {
                todo!("Implement different pixel formats");
            };

            if format.stride % 2 != 0 {
                todo!("How do we deal with strides that are not multiples of the pixel size?");
            }

            format!("
constexpr size_t {}_width {{{}}};
constexpr size_t {}_height {{{}}};
inline uint16_t volatile (&{}_pixels)[{}_height][{}] {{*reinterpret_cast<uint16_t volatile (*)[{}_height][{}]>({:#x})}};

",
                    name, format.width,
                    name, format.height,
                    name, name, format.stride / 2, name, format.stride / 2, resource.opt_region.as_ref().expect("framebuffer without memory region").virt_start)
        }
        runtypes::ResourceMetaInfo::SBITimer { freq_hz } => format!(
            "
constexpr uint64_t {}_freq_hz {{{}}};
",
            name, freq_hz
        ),
        runtypes::ResourceMetaInfo::SpinalGPIO { ngpio } => format!(
            "
constexpr uint16_t {}_ngpio {{{}}};
inline uint32_t volatile * const {}_reg {{reinterpret_cast<uint32_t volatile *>({:#x}ul)}};
",
            name,
            ngpio,
            name,
            resource
                .opt_region
                .as_ref()
                .expect("PLIC without memory region")
                .virt_start
        ),
    }
}

pub fn generate(language: Language, process: &runtypes::Process) -> String {
    match language {
        Language::CPP => format!(
            "// Automatically generated. Do not touch.

#pragma once

#if __STDC_HOSTED__
#include <cstddef>
#include <cstdint>
#else
#include <epoxy-api/c_types.hpp>
#endif

{}",
            process
                .resources
                .iter()
                .map(|(name, res)| generate_cpp_res(name, res))
                .join("\n")
        ),
    }
}
