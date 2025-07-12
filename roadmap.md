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
- [ ] Display updated standings.

### Step 3: **Polish for MVP**
- [ ] Make the UI user-friendly and not ugly.
- [ ] Test all features.
- [ ] Write a clear README (explain what, why, and how).
- [ ] Record a short pitch/demo video.

---

## **How to Write the README**

- **Project Title and Description:** What is Quasar Cup? What problem does it solve?
- **Features:** List the core features you implemented.
- **How to Run:** Step-by-step instructions to run backend and frontend.
- **How to Use:** Example workflow (create tournament, add teams, generate schedule, enter results).
- **Tech Stack:** Rust (Axum) backend, Yew frontend, etc.
- **Future Ideas:** List things you want to add if you keep working on it.

---

## **Keep Motivated**
Remember, every time you get a feature working — even a small one — that’s a **win**! Celebrate it. Maybe keep a “changelog” in your README or a notebook so you can see your progress.

---

## 7. **Next Small Steps**
**Right now:**  
- Finish the flow of creating a tournament, adding teams, and generating a fixture.  
- Get the frontend and backend talking smoothly.

**Once that works:**  
- Add match result submission.
- Standings table.

---

### 🚀 You’re already making great progress! Each little step is a building block toward your big idea. Keep going, and remember — you’re learning a ton, and that’s the real win here!

---
