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

//! A collection of handlers for the HTTP server's router

use bodyparser;
use hab_net::http::controller::*;
use hab_net::routing::Broker;
use iron::prelude::*;
use iron::status;
use protocol::search::FromSearchPair;
use protocol::sessionsrv::*;
use router::Router;
use serde_json::Value;

pub fn account_features_show(req: &mut Request) -> IronResult<Response> {
    Ok(Response::with(status::Ok))
}

pub fn account_show(req: &mut Request) -> IronResult<Response> {
    let params = req.extensions.get::<Router>().unwrap();
    let id = params.find("id").unwrap();
    if id.parse::<u64>().is_err() {
        return Ok(Response::with(status::BadRequest));
    }
    let search = AccountSearch::from_search_pair("id", id).unwrap();
    let mut conn = Broker::connect().unwrap();
    match conn.route::<AccountSearch, Account>(&search) {
        Ok(account) => Ok(render_json(status::Ok, &account)),
        Err(err) => Ok(render_net_error(&err)),
    }
}

pub fn feature_show(req: &mut Request) -> IronResult<Response> {
    Ok(Response::with(status::Ok))
}

pub fn feature_update(req: &mut Request) -> IronResult<Response> {
    Ok(Response::with(status::Ok))
}

pub fn features_list(req: &mut Request) -> IronResult<Response> {
    Ok(Response::with(status::Ok))
}

/// Endpoint for determining availability of builder-api components.
///
/// Returns a status 200 on success. Any non-200 responses are an outage or a partial outage.
pub fn status(_req: &mut Request) -> IronResult<Response> {
    Ok(Response::with(status::Ok))
}

pub fn search(req: &mut Request) -> IronResult<Response> {
    match req.get::<bodyparser::Json>() {
        Ok(Some(ref body)) => {
            let attr = match body.find("attr") {
                Some(&Value::String(ref s)) => s.to_string(),
                _ => return Ok(Response::with(status::UnprocessableEntity)),
            };
            let value = match body.find("value") {
                Some(&Value::String(ref s)) => s.to_string(),
                _ => return Ok(Response::with(status::UnprocessableEntity)),
            };
            match body.find("entity") {
                Some(&Value::String(ref s)) if &*s == "account" => search_account(attr, value),
                _ => Ok(Response::with(status::UnprocessableEntity)),
            }
        }
        _ => Ok(Response::with(status::BadRequest)),
    }
}

fn search_account(key: String, value: String) -> IronResult<Response> {
    match AccountSearch::from_search_pair(key, value) {
        Ok(search) => {
            let mut conn = Broker::connect().unwrap();
            match conn.route::<AccountSearch, Account>(&search) {
                Ok(account) => Ok(render_json(status::Ok, &account)),
                Err(err) => Ok(render_net_error(&err)),
            }
        }
        Err(err) => Ok(Response::with((status::UnprocessableEntity, err.to_string()))),
    }
}
