# tamako

> (wip) basic backend implemented. missing frontend.

cozy anonymous whispers. 🐞

# todo!

- [x] implement api endpoints
  - [x] add whisper `POST @ /api/whisper`
  - [x] list whispers `GET @ /api/whispers`
  - [ ] delete whisper/s (requires auth) `DELETE @ /api/whispers/:snowflake`
- [x] limit payload to prevent spam
- [ ] web frontend (literal hell)
  - [x] basic skeleton
  - [ ] actual functionality
  - [ ] cleanup code somehow
- [ ] implement ratelimit
- [ ] github oauth to access private whispers `/api/auth/github`
- [ ] dockerize
- [ ] optional non-anonymous whispers
- [ ] discord webhook support
- [ ] simple cli utility
