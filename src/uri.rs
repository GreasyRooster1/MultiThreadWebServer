mod uri {
    // find a file path from a uri
    fn find(uri: &str) -> String {
        //this should return "data/foo.html" (or whatever else, based on the URI)
        //however, not sure if uri's ownership is kept by whoever owned it before the uri::find() call
        "data/".to_owned().clone() + &*uri
    }

    fn extract(http_request: &str) -> &str {
        let line = http_request.lines().next().unwrap();
        // return uri (remove GET prefix and HTTP/1.1 suffix)
        line.strip_prefix("GET").unwrap().strip_suffix("HTTP/1.1").unwrap()
    }
}