// The MIT License (MIT)
// Copyright (c) 2023 IBP.network
//
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in all
// copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.

use crate::api::guards::ApiKeyGuard;
use crate::api::handlers::alerts::post_alert;
use crate::api::handlers::index::get_index;
use actix_web::web;

/// All routes are placed here
pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg
        // Index
        .route("/", web::get().to(get_index))
        // /api/v1 routes
        .service(
            web::scope("/api/v1")
                .guard(ApiKeyGuard)
                // API info
                .route("", web::get().to(get_index))
                // Alerts route
                .route("/alerts", web::post().to(post_alert)),
        );
}
