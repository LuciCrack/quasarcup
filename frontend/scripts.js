const API_BASE = "http://localhost:8000";

// Handle create tournament form
const createForm = document.getElementById('create-form');
if (createForm) {
    createForm.addEventListener('submit', async (event) => {
        event.preventDefault(); // Stop form from refreshing page
        
        const formData = new FormData(createForm);
        const tournamentData = {
            tournament_name: formData.get('name'),
            team_number: parseInt(formData.get('number'), 10)
            // Add other fields from your form
        };
        
        try {
            const response = await fetch(`${API_BASE}/api/create_tournament`, {
                method: 'POST',
                headers: {
                    'Content-Type': 'application/json',
                },
                body: JSON.stringify(tournamentData),
            });
            
            if (response.ok) {
                const result = await response.json();
                // Redirect to tournament page with the code
                window.location.href = `tournament.html?code=${result}`;
            } else {
                alert('Error creating tournament');
            }
        } catch (error) {
            console.error('Error:', error);
            alert('Failed to fetch the create tournament api');
        }
    });
}

// Handle access tournament form
const accessForm = document.getElementById('access-form');
if (accessForm) {
    accessForm.addEventListener('submit', (event) => {
        event.preventDefault();
        const code = accessForm.querySelector('input').value.toUpperCase();
        window.location.href = `tournament.html?code=${code}`;
    });
}

// Load and display tournament data
async function loadTournament() {
    const urlParams = new URLSearchParams(window.location.search);
    const code = JSON.stringify(urlParams.get('code').trim().replace(/^"|"$/g, ""));
    
    if (!code) {
        document.body.innerHTML = '<p>No tournament code provided</p>';
        return;
    }
    
    try {
        // Ask if the tournament exists before fetching
        const exists = await fetch(`${API_BASE}/api/exists_tournament`, {
            method: 'POST',
            headers: {
                'Content-Type': 'application/json',
            },
            body: code
        });

        if (!exists.ok || !exists.json()) {
            document.body.innerHTML = '<p>Tournament not found</p>';
            return;
        }

        // Once we have confirmation we render the tournament
        const response = await fetch(`${API_BASE}/api/get_tournament`, {
            method: 'POST',
            headers: {
                'Content-Type': 'application/json',
            },
            body: code
        });
        if (response.ok) {
            const tournament = await response.json();
            displayTournament(tournament);
        }
    } catch (error) {
        console.error('Error:', error);
        document.body.innerHTML = '<p>Error loading tournament</p>';
    }

//            
}

function displayTournament(tournament) {
    const content = `
        <h1>${tournament.name}</h1>
        ${Object.entries(tournament.matches).map(([date, games]) => `
            <h3>Date ${date}</h3>
            ${games.map(match => `
                <div class="match">
                    ${match.home_team.name} vs ${match.away_team.name}
                    ${match.home_score ?? 0} - ${match.away_score ?? 0}
                </div>
            `).join('')}
        `).join('')}
    `;

    document.body.innerHTML = content;
}

// Load tournament if we're on the tournament page
if (window.location.pathname.includes('tournament.html')) {
    loadTournament();
}
