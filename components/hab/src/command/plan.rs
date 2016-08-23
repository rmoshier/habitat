// Copyright (c) 2016 Chef Software Inc. and/or applicable contributors
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

pub mod create {
    use std::io::prelude::*;
    use std::fs::{create_dir, File};
    use std::path::PathBuf;

    use error::Result;

    pub fn start(plan_name: &str) -> Result<()> {
        println!("creating package {}", plan_name);

        // 0. create a new directory IF it does not already exist
        // 1. create a plan.sh file from a template, possibly
        //    using the current HAB_ORIGIN environment value

        // output:
        // hab pkg create foo
        // ./foo
        // ./foo/plan.sh
        let mut path = PathBuf::from(".");
        path.push(&plan_name);
        if path.is_dir() {
                panic!("PLAN DIRECTORY ALREADY EXISTS");
        }
        try!(create_dir(&path));
        path.push("plan.sh");
        let mut f = try!(File::create(&path));

        let plan_template =
format!(
"pkg_name={}
pkg_origin=core
pkg_version=0.1.0
pkg_description=\"\"
pkg_upstream_url=
pkg_license=('Apache-2.0')
pkg_maintainer=\"The Habitat Maintainers <humans@habitat.sh>\"
pkg_source=http://example.com/${{pkg_name}}-${{pkg_version}}.tar.gz
pkg_shasum=
pkg_bin_dirs=(bin)
pkg_build_deps=(core/make core/gcc)
pkg_deps=(core/glibc)
", &plan_name);

        try!(f.write_all(&plan_template.as_bytes()));
        Ok(())
    }
}
