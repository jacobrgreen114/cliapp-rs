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

use crate::constexpr::Argument;

/// The value that will be set when a parameter is present on the command line.
pub struct ParameterValue {
    value: std::cell::UnsafeCell<Option<String>>,
}

impl ParameterValue {
    pub const fn new() -> Self {
        Self {
            value: std::cell::UnsafeCell::new(None),
        }
    }

    pub fn value(&self) -> Option<&str> {
        unsafe { (&*self.value.get()).as_ref().map(|s| s.as_str()) }
    }

    fn set_value(&self, value: String) {
        unsafe {
            self.value.get().replace(Some(value));
        }
    }
}

unsafe impl Sync for ParameterValue {}

/// A command line string parameter.
///
/// # Example
/// ```bash
/// $ ./myapp -{short_name} {value} --{long_name}={value}
/// ```
pub struct Parameter<'a> {
    short_name: &'a str,
    long_name: &'a str,
    description: &'a str,
    value: &'a ParameterValue,
}

impl<'a> Parameter<'a> {
    pub const fn build() -> ParameterBuilder<'a> {
        ParameterBuilder {
            short_name: None,
            long_name: None,
            description: None,
            parameter: None,
        }
    }

    pub(crate) fn set_value(&self, value: String) {
        self.value.set_value(value);
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

impl Argument for Parameter<'_> {
    fn long_name(&self) -> &str {
        self.long_name
    }

    fn short_name(&self) -> &str {
        self.short_name
    }
}


pub struct ParameterBuilder<'a> {
    short_name: Option<&'a str>,
    long_name: Option<&'a str>,
    description: Option<&'a str>,
    parameter: Option<&'a ParameterValue>,
}

impl<'a> ParameterBuilder<'a> {
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

    pub const fn with_parameter(mut self, value: &'a ParameterValue) -> Self {
        self.parameter = Some(value);
        self
    }

    pub const fn build(self) -> Parameter<'a> {
        let param = Parameter {
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
            value: match self.parameter {
                Some(value) => value,
                None => panic!("Parameter must have a value."),
            },
        };
        assert!(
            !(param.short_name.is_empty() && param.long_name.is_empty()),
            "Parameter must at least have a short name or long name."
        );
        param
    }
}
