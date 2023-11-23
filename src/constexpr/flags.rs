/*
 * Copyright 2023 Jacob R. Green
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 * http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

/// The value that will be set when a flag is present on the command line.
pub struct FlagValue {
    value: std::cell::Cell<bool>,
}

impl FlagValue {
    pub const fn new() -> Self {
        Self {
            value: std::cell::Cell::new(false),
        }
    }

    pub fn value(&self) -> bool {
        self.value.get()
    }

    fn mark(&self) {
        self.value.set(true)
    }
}

unsafe impl Sync for FlagValue {}

/// A command line boolean flag.
///
/// # Example
/// ```bash
/// $ ./myapp -{short_name} --{long_name}
/// ```
pub struct Flag<'a> {
    short_name: &'a str,
    long_name: &'a str,
    description: &'a str,
    flag: &'a FlagValue,
}

impl<'a> Flag<'a> {
    pub const fn build() -> FlagBuilder<'a> {
        FlagBuilder {
            short_name: None,
            long_name: None,
            description: None,
            flag: None,
        }
    }

    pub(crate) fn mark(&self) {
        self.flag.mark();
    }

    pub const fn short_name(&self) -> &str {
        self.short_name
    }

    pub const fn long_name(&self) -> &str {
        self.long_name
    }

    pub const fn description(&self) -> &str {
        self.description
    }
}

pub struct FlagBuilder<'a> {
    short_name: Option<&'a str>,
    long_name: Option<&'a str>,
    description: Option<&'a str>,
    flag: Option<&'a FlagValue>,
}

impl<'a> FlagBuilder<'a> {
    pub const fn with_short_name(mut self, short_name: &'a str) -> Self {
        self.short_name = Some(short_name);
        self
    }

    pub const fn with_long_name(mut self, long_name: &'a str) -> Self {
        self.long_name = Some(long_name);
        self
    }

    pub const fn with_description(mut self, description: &'a str) -> Self {
        self.description = Some(description);
        self
    }

    pub const fn with_flag(mut self, flag: &'a FlagValue) -> Self {
        self.flag = Some(flag);
        self
    }

    pub const fn build(self) -> Flag<'a> {
        let flag = Flag {
            short_name: match self.short_name {
                Some(short_name) => short_name,
                None => "",
            },
            long_name: match self.long_name {
                Some(long_name) => long_name,
                None => "",
            },
            description: match self.description {
                Some(description) => description,
                None => "",
            },
            flag: match self.flag {
                Some(flag) => flag,
                None => panic!("Flag must have a flag value."),
            },
        };
        assert!(
            !(flag.short_name.is_empty() && flag.long_name.is_empty()),
            "Flag must at least have a short name or long name."
        );
        flag
    }
}
