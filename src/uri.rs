use crate::{files, paths};
use crate::files::load_contents;
use crate::logging::log_debug;
use crate::paths::{DATA_PATH, DEFAULT_PATH, HIDDEN_PATH};

pub(crate) fn find(uri: &str) -> String {
    //this should return "data/foo.html" (or whatever else, based on the URI) and keep uri's owner the same
    DATA_PATH.to_owned().clone() + uri
}
pub(crate) fn find_hidden(uri: &str) -> String {
    //this should return "hidden/foo.html" (or whatever else, based on the URI) and keep uri's owner the same
    HIDDEN_PATH.to_owned().clone() + uri
}
pub(crate) fn extract(http_request: &str) -> &str {
    let line = http_request.lines().next().unwrap();
    // return uri (remove GET prefix and HTTP/1.1 suffix)
    line.strip_prefix("GET")
        .unwrap()
        .strip_suffix("HTTP/1.1")
        .unwrap()
        .trim()
}
pub(crate) fn extension(filename: &str) -> &str {
    // get everything after the '.' char
    &filename[filename.rfind('.').unwrap_or(filename.len())..filename.len()]
}
pub(crate) fn parse(uri: &str) -> String {
    if extension(uri).is_empty() {
        uri.to_owned().clone() + ".html"
    } else {
        uri.parse().unwrap()
    }
}

pub(crate) fn get_mime_type(filetype:&str)->String{
    match filetype {
        ".html"=>{"text/html".to_string()}
        ".css"=>{"text/css".to_string()}
        ".js"=>{"text/javascript".to_string()}
        ".mjs"=>{"text/javascript".to_string()}
        ".ico"=>{"image/vnd.microsoft.icon".to_string()}
        ".png"=>{"image/png".to_string()}
        ".jpg"=>{"image/jpeg".to_string()}
        _ => {"".to_string()}
    }
}
pub struct HTTPResponse{

}
impl HTTPResponse {
    pub(crate) fn from_bytes(status: String, contents: &[u8]) -> Vec<u8> {
        let len = contents.len();
        [format!("HTTP/1.1 {status}\r\nContent-Length:{len}\r\n\r\n").as_bytes().to_owned().clone().as_slice(), contents].concat()
    }
    pub(crate) fn from_string(status: &str, contents: &str) -> String {
        let len = contents.len();
        format!("HTTP/1.1 {status}\r\nContent-Length:{len}\r\n\r\n{contents}")
    }
    pub(crate) fn from_string_as_bytes(status: &str, contents: &str) -> Vec<u8> {
        let len = contents.len();
        format!("HTTP/1.1 {status}\r\nContent-Length:{len}\r\n\r\n{contents}").as_bytes().to_owned()
    }
    pub(crate) fn from_bytes_with_extras(status: String,extras:Vec<String>, contents: &[u8])->Vec<u8>{
        let len = contents.len();
        [format!("HTTP/1.1 {status}\r\n{0}\r\nContent-Length:{len}\r\n\r\n",extras.as_slice().join("\r\n")).as_bytes().to_owned().clone().as_slice(), contents].concat()
    }
    pub(crate) fn from_bytes_with_mime(status: String,mime:String, contents: &[u8]) -> Vec<u8>{
        let extras = vec![format!("Content-Type: {mime}")];
        Self::from_bytes_with_extras(status,extras,contents)
    }
    pub(crate) fn from_uri(uri:&str) -> Vec<u8> {
        let filename = if uri.eq("/") {
            find(DEFAULT_PATH)
        }else{
            parse(find(uri).as_str())
        };
        let (contents,status) = if files::file_exists(filename.as_str()) {
            (load_contents(filename.as_str()),"200 OK")
        }else{
            (load_contents(paths::NOT_FOUND_PATH),"NOT FOUND")
        };
        let mime = get_mime_type(parse(extension(uri)).as_str());
        log_debug(format!("mime type is: {}", mime).as_str(),"worker");
        HTTPResponse::from_bytes_with_mime(status.to_string(),mime,contents.as_slice())
    }
}

//tests
#[cfg(test)]
mod tests {
    use crate::uri;
    #[test]
    fn find() {
        assert_eq!(uri::find("/index.html"), "data/index.html");
        assert_eq!(uri::find("/foo/bar.html"), "data/foo/bar.html");
        assert_eq!(uri::find("/"), "data/");

        let uri: &str = "/index.html";
        assert_eq!(uri::find(uri), "data/index.html");
        assert_eq!(uri, "/index.html");

        let mut uri_mut: &str = "/foo.html";
        assert_eq!(uri::find(uri_mut), "data/foo.html");
        assert_eq!(uri_mut, "/foo.html");
        uri_mut = "/bar.html";
        assert_eq!(uri::find(uri_mut), "data/bar.html");
        assert_eq!(uri_mut, "/bar.html");
    }
    #[test]
    fn extract() {
        assert_eq!(uri::extract("GET / HTTP/1.1"), "/");
        assert_eq!(uri::extract("GET /index.html HTTP/1.1"), "/index.html");
        assert_eq!(uri::extract("GET /foo/bar.html HTTP/1.1"), "/foo/bar.html");
        assert_eq!(uri::extract("GET /GET/HTTP/1.1 HTTP/1.1"), "/GET/HTTP/1.1");
    }
    #[test]
    fn extension() {
        assert_eq!(uri::extension("index.html"), ".html");
        assert_eq!(uri::extension("script.js"), ".js");
        assert_eq!(uri::extension("no-extension"), "");
        assert_eq!(uri::extension("mult.aple.ext.ensio.ns.html"), ".html");
    }
}
