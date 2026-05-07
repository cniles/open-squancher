# appointment-scraper

Polls an appointment availability API every 5 minutes and prints any open slots that fall before a given date.

## Usage

```
appointment-scraper <endpoint> <before-date>
```

- `<endpoint>` — URL returning a JSON array of appointment objects (see below)
- `<before-date>` — only report slots before this date, in `MM-DD-YYYY` format

**Example:**

```sh
appointment-scraper https://example.com/api/appointments 06-01-2026
```

Output:

```
Watching for open appointments before 2026-06-01 — checking every 5 minutes.
[2026-05-07 09:00:00] No open appointments found before 2026-06-01.
[2026-05-07 09:05:00] Open appointments found:
  2026-05-15 — 2 open slot(s)
```

## Expected API response

The endpoint must return a JSON array where each object has:

```json
[
  { "apptDate": "2026-05-15", "open": 2 },
  { "apptDate": "2026-05-20", "open": 0 }
]
```

- `apptDate` — date string in `YYYY-MM-DD` format
- `open` — number of available slots (0 means fully booked)

## Build & run

```sh
cargo build --release
./target/release/appointment-scraper <endpoint> <before-date>
```
