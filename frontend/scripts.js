const API_BASE = 'http://localhost:8000';

// Handle create tournament form
const createForm = document.getElementById('create-form');
if (createForm) {
    createForm.addEventListener('submit', async (event) => {
        event.preventDefault(); // Stop form from refreshing page
        
        const formData = new FormData(createForm);
        const tournamentData = {
            name: formData.get('name'),
            players: parseInt(formData.get('players'))
            // Add other fields from your form
        };
        
        try {
            const response = await fetch(`${API_BASE}/api/tournaments`, {
                method: 'POST',
                headers: {
                    'Content-Type': 'application/json',
                },
                body: JSON.stringify(tournamentData)
            });
            
            if (response.ok) {
                const result = await response.json();
                // Redirect to tournament page with the code
                window.location.href = `tournament.html?code=${result.code}`;
            } else {
                alert('Error creating tournament');
            }
        } catch (error) {
            console.error('Error:', error);
            alert('Failed to create tournament');
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
    const code = urlParams.get('code');
    
    if (!code) {
        document.body.innerHTML = '<p>No tournament code provided</p>';
        return;
    }
    
    try {
        const response = await fetch(`${API_BASE}/api/tournaments/${code}`);
        if (response.ok) {
            const tournament = await response.json();
            displayTournament(tournament);
        } else {
            document.body.innerHTML = '<p>Tournament not found</p>';
        }
    } catch (error) {
        console.error('Error:', error);
        document.body.innerHTML = '<p>Error loading tournament</p>';
    }
}

function displayTournament(tournament) {
    const content = `
        <div class="top-bar">
            <h1>${tournament.name}</h1>
            <a href="index.html">Home</a>
        </div>
        <div class="main-content">
            <h2>Matches</h2>
            <div id="matches">
                ${tournament.matches.map(match => `
                    <div class="match">
                        ${match.player1} vs ${match.player2} 
                        - Score: ${match.score1 ?? 0} : ${match.score2 ?? 0}
                    </div>
                `).join('')}
            </div>
        </div>
    `;
    document.body.innerHTML = content;
}

// Load tournament if we're on the tournament page
if (window.location.pathname.includes('tournament.html')) {
    loadTournament();
}
