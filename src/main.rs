/*
 * The contents of this file are subject to the terms of the
 * Common Development and Distribution License, Version 1.1 only
 * (the "License").  You may not use this file except in compliance
 * with the License.
 *
 * See the file LICENSE in this distribution for details.
 * A copy of the CDDL is also available via the Internet at
 * https://spdx.org/licenses/CDDL-1.1.html
 *
 * When distributing Covered Code, include this CDDL HEADER in each
 * file and include the contents of the LICENSE file from this
 * distribution.
 */

use actix_web::{get, http::header, web, App, HttpResponse, HttpServer};
use clap::Parser;
use comrak::{markdown_to_html, ComrakOptions};
use orgize::Org;
use std::{env, ffi::OsStr, fs, io::Read, path::Path, process};
use zip::ZipArchive;

#[derive(Parser)]
#[clap(version, about, long_about = None)]
struct Args {
    #[clap(long, help = "Set the port on which to run", default_value_t = 8000)]
    port: u16,

    #[clap(long, help = "Set the standard index page", default_value = "index")]
    defaultpage: String,
}

async fn dispatch(filepath: &str) -> HttpResponse {
    // Builds a response, based on the file's extension inside
    // the ZIP file.
    let args: Vec<_> = env::args().collect();

    // This is where the magic happens:
    let zipfile = fs::File::open(Path::new(&args[0])).unwrap();
    let mut archive = ZipArchive::new(&zipfile).unwrap();

    // Now the problem is that, if we call /index, it can be a file
    // named "index.org" as well. Try to determine whether that
    // exists:
    let html_extensions = ["md", "org", "htm", "html"];
    let potential_files = [
        filepath.to_string(),
        format!("{}.md", filepath),
        format!("{}.org", filepath),
        format!("{}.htm", filepath),
        format!("{}.html", filepath),
    ];

    let extension: String;

    for f in archive.clone().file_names() {
        for potential_file in &potential_files {
            if f == potential_file {
                // This is the file we want.
                // Get a handle on the file:
                let mut file = archive.by_name(f).unwrap();

                // Determine the file extension:
                extension = String::from(
                    Path::new(f)
                        .extension()
                        .and_then(OsStr::to_str)
                        .ok_or("")
                        .unwrap(),
                );

                if (&html_extensions).iter().any(|e| e == &extension) {
                    // This is a HTML file.
                    let mut contents = String::new();
                    let _ = file.read_to_string(&mut contents);

                    // - .md: Pass to the Markdown parser.
                    // - .org: Pass to the org-mode parser.
                    // - .htm(l): Just pass it to the client (as HTML).
                    let html_output = match extension.as_str() {
                        "md" => md_to_html(&contents),
                        "org" => org_to_html(&contents),
                        "htm" => contents.clone(),
                        "html" => contents.clone(),
                        &_ => "".to_string(), // This will not happen.
                    };

                    return HttpResponse::Ok()
                        .insert_header(header::ContentType(mime::TEXT_HTML_UTF_8))
                        .body(html_output);
                } else {
                    // If we reach this point, the requested file is not
                    // a (supported) HTML file. Let's return a $ContentType
                    // file instead and pass it to the client statically.
                    let mut contents = Vec::new();
                    let _ = file.read_to_end(&mut contents);
                    return HttpResponse::Ok()
                        .insert_header(header::ContentType(
                            mime_guess::from_ext(&extension).first_or_text_plain(),
                        ))
                        .body(contents);
                }
            }
        }
    }

    // Else, file not found.
    HttpResponse::NotFound().finish()
}

fn md_to_html(in_str: &str) -> String {
    // Converts the Markdown input string <in> to HTML.
    markdown_to_html(in_str, &ComrakOptions::default())
}

fn org_to_html(in_str: &str) -> String {
    // Converts the .org input string <in> to HTML.
    let mut writer = Vec::new();
    Org::parse(in_str).write_html(&mut writer).unwrap();
    String::from_utf8(writer).unwrap()
}

// --------- ROUTING ---------

#[get("/")]
async fn index() -> HttpResponse {
    let argv = Args::parse();
    dispatch(&argv.defaultpage).await
}

#[get("/{file}")]
async fn get_file(file: web::Path<String>) -> HttpResponse {
    let filepath = file.into_inner();
    dispatch(&filepath).await
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Sanity check: Do we even have a Zip archive?
    // (This cannot be changed while the application is running,
    //  so we won't have to check this later anymore.)
    let args: Vec<_> = env::args().collect();
    let zipfile = fs::File::open(Path::new(&args[0])).unwrap();
    match ZipArchive::new(&zipfile) {
        Err(_) => {
            eprintln!("There is no Zip archive here. Exiting.");
            process::exit(42);
        }
        Ok(_) => println!("Found a valid Zip archive."),
    };

    // Start and listen:
    let argv = Args::parse();
    println!("Starting zb on port {}.", argv.port);

    HttpServer::new(|| App::new().service(index).service(get_file))
        .bind(("0.0.0.0", argv.port))?
        .run()
        .await
}
