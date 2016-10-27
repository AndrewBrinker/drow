fn main() {
    // Drow should:
    //
    // 1. Verify that you are currently in a drow project (identified by the
    //    presence of a `Drow.toml` file in a parent directory).
    // 2. Parse the configuration in the `Drow.toml` file and ensure that it
    //    is a valid configuration, providing useful error messages otherwise.
    // 3. Verify that the site directory is present.
    //
    // CLI API
    //
    // drow setup  - Create a new Drow project
    // drow run    - Serve your Drow project locally
    // drow build  - Build your Drow project once
    // drow deploy - Deploy your Drow project
    // drow post   - Create a new post and open your default editor
    // drow page   - Create a new page and open your default editor
    //
    // Should probably draw inspiration from the internals of Cargo.
}
