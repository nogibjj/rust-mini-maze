# US Road Information CLI

This is a Rust CLI project that provides information about major US roads and highways.

## Usage
To use the CLI, run the following command:
```
us-road-info <road-number>
```

Replace <road-number> with the number of the road you want information about.

For example:
```
us-road-info 1
```
This will provide information about Route 1, including its starting and ending points.

## Example
```
Enter a road name:
```
```
1
```
```
1. Route 1
2. Interstate 10
3. US 1
4. US 101
5. US 129
6. US 191
7. US 212
Enter a road number:
```
```
7
```
```
US 212: Yellowstone National Park, WY - Sioux Falls, SD
```

## Data
The CLI uses a JSON file to store data about the roads. The data includes the name of the road and its starting and ending points.

## Installation
To install the CLI, clone this repository and run the following command:
```
cargo install --path .
```

## References

* [rust-cli-template](https://github.com/kbknapp/rust-cli-template)
