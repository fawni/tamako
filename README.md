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
- [ ] cleanup code
  - [ ] backend
  - [ ] frontend
- [ ] implement ratelimit
- [ ] implement auth for private whispers
  - [ ] /auth to login
  - [ ] /api/auth takes password and returns hash then save hash as token to local storage
  - [ ] inclute header auth w token in requests
  - [ ] return private whispers if token matches password
  <!-- - [ ] github oauth to access private whispers `/api/auth/github` -->
- [ ] dockerize
- [ ] optional non-anonymous whispers
- [ ] discord webhook support
- [ ] simple cli utility
