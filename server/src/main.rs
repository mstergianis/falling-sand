use warp::Filter;

#[tokio::main]
async fn main() {
    // Define the directory to serve files from
    let static_files = warp::fs::dir("static");

    // Create a warp filter that serves files from the specified directory
    let routes = static_files;

    // Start the server on localhost:3030
    println!("Serving static files on http://localhost:3030");
    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await
}
