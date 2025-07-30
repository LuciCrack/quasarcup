---

_"Quasar Cup is a web application to help anyone easily organize soccer tournaments. It handles team creation, match scheduling, results, and stats, so organizers can focus on the fun part — the games!"_

---

## **Minimum Viable Product (MVP) Roadmap**

### A. **Core Features (Must-have for CS50x Final Project)**
1. **User can create a new tournament.**
   - Input tournament name, number of teams.
2. **User can add teams (with names at least).**
3. **App generates a fixture (schedule) automatically.**
   - Round-robin for a number of teams.
   - Display dates and matches for each round.
4. **User can enter results for each match.**
   - Input scores for both teams.
5. **Standings Table**
   - Show team stats: games played, won, drawn, lost, points, goals for/against, etc.
6. **Basic stats tracking for teams (and maybe players, if time allows).**
7. **Simple, easy-to-use web interface.**
   - No fancy design required, but clear and usable.
8. **README and Pitch Video**  
   - Explain what it does, how to use, and a demo.

### B. **Nice-to-Have/Stretch Goals **
- **Edit teams after creation.**
- **Edit match dates or results.**
- **Basic player management (add players to teams, track goals).**
- **Multiple tournament formats (single/double round-robin, knockout, group stage).**
- **Custom points system.**

### C. **Future Ideas (Not for MVP)**
- User accounts (organizers, viewers)
- Cloud data storage & sharing
- Team generator (random or criteria-based)
- Multi-language support (English/Spanish)
- Media uploads (photos, videos)
- Drag and drop UI for fixtures or teams

---

## **How to Get There: Step-by-Step Plan**

### Step 1: **Backend Basics**
- [x] Set up basic Rust backend with Axum.
- [x] Define data models: Tournament, Team, Match/Game, Fixture, etc.
- [x] Implement API endpoints:
  - Create tournament
  - Create/generate fixture
  - Enter results
  - Get standings

### Step 2: **Frontend Basics**
- [x] Set up Yew frontend.
- [x] Create forms for tournament and team creation.
- [x] Display generated fixture (schedule).
- [x] Form to submit results.
- [x] Display updated standings.

### Step 3: **Polish for MVP**
- [ ] Make the UI user-friendly and not ugly.
- [ ] Test all features.
- [ ] Write a clear README (explain what, why, and how).
- [ ] Record a short pitch/demo video.
- [ ] Save tournaments to cloud and be able to access later

---

## **How to Write the README**

- **Project Title and Description:** What is Quasar Cup? What problem does it solve?
- **Features:** List the core features you implemented.
- **How to Run:** Step-by-step instructions to run backend and frontend.
- **How to Use:** Example workflow (create tournament, add teams, generate schedule, enter results).
- **Tech Stack:** Rust (Axum) backend, Yew frontend, etc.
- **Future Ideas:** List things you want to add if you keep working on it.

---

## **Right Now**

### **Endpoints and backend-frontend flow**
A. Create Tournament

    POST /tournament
        Body: { name, team_number }
        Backend: Save to DB, generate fixture, store, return { tournament_id: 1234 } (or code like P4X9ZQ).

B. View Tournament

    GET /tournament?id=1234
        Backend: Fetch from DB by ID or code, return all tournament data (teams, fixtures, results).

C. Search Tournament

    GET /search?code=XXXXXX
        Backend: Look up by code, redirect or return not found.

D. Update Tournament

    POST /tournament/update_match
        Body: { tournament_id, date_idx, game_idx, home_score, away_score }
        Backend: Update DB record for that match.

E. TODO: more features

### **sqlite schema**

    tournaments table: id, name, code, created_at
    teams table: id, tournament_id, name
    games table: id, tournament_id, date_idx, game_idx, home_team_id, away_team_id, home_score, away_score

