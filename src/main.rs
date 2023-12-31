use std::borrow::Cow;
use std::collections::HashMap;
use std::num::ParseIntError;
use std::os::raw::c_int;
use AthenaEngine::log::{log_text_writer, LogTypeTag};

use AthenaEngine::server;
use AthenaEngine::server::page_manager::page_manager;
use AthenaEngine::server::page_manager::page_manager::{GetPageTemplateVar, page_template_parser, PageInfo};
use AthenaEngine::server::request_parser::request_parser::{HttpVersion, Method};
use AthenaEngine::server::request_parser::request_parser::Method::POST;
use AthenaEngine::server::response_parser::response_parser::{default_response_writer, IsResponseDataCreateSuccess, Response, ResponseBody, ResponseCookies};

static mut HTTP_URL_REDIRECT_WARNING_API: &str = "/api/redirect_warning";

fn main() {
    // All pages hashmap
    let mut all_page_list: HashMap<String, PageInfo> = HashMap::new();


    // Loading redirect API
    // ================================
    let util_redirect_warning_api : PageInfo = PageInfo {
        file_path: "/resources/pages/api/util_redirect_warning.html".to_string(), // HTML file path
        is_access: true // File accessibility
    };
    // ================================
    unsafe {
        all_page_list.insert(String::from(HTTP_URL_REDIRECT_WARNING_API), util_redirect_warning_api);
    }
    // ================================


    // All pages list setting
    unsafe {
        page_manager::ALL_PAGES.pages = Some(all_page_list);
    }


    // Client connection event setting
    unsafe {
        // Request event setting
        server::EVENT.event_request = Some(Box::new(|request| {

        }));

        // Response event setting
        server::EVENT.event_response = Some(Box::new(|request| {
            // Default response packet
            let mut response : Response = default_response_writer(&request, None, None);

            // 작업 확인
            match &request.target {
                None => {}
                Some(target) => {
                }
            }

            // Return response
            return response;
        }));
    }

    // Open server
    server::start_server(String::from("0.0.0.0"), 8080);
}