# Lifestyle
This is a website and server that is intended to be used to track and display progress for a number of fields. Currently these fields are intended to be calorie intake and weight loss, progress towards larger price items, Valorant rank with the intention of reaching gold 1, and the current progress of this project. This project is intended to be run on a raspberry pi. At this time this website is only intended to be used by a single person but this might be changed to allow multiple people to have their own profiles.

## What's going on behind the scenes:
- A multi threaded web server is serving the website, each of the categories that are being tracked are their own structs. The user can add information to what the server contains via the website.
- The data is currently being saved every time the server shuts down and is saved in plain text files. 
- The content of the sub pages are rendered on the server side.

## What works?
- The overview page properly displays data properly.
- Calorie and weight tracking is feature complete.
- Adding new bugs/features in the progress section works from client to server.

## Current Issues
- There is no way to edit the goals.
    - A button should be added to the overview that allows a user to modify values.
- Feature progress is not visible on client side.
    - No way to delete or mark as complete.
- Shopping and Valorant haven't been touched.

## How you can run this project:
- Make sure that you have both Rust and Typescript installed.
```
git clone https://github.com/nuhtan/lifestyle.git
cargo build --release
./target/release/main
```