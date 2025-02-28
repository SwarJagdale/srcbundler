import { useGame } from "../context/GameContext"

const Gameover = ({ won, correctWord, attempts, onNewGame }) => {
  const { gameStats } = useGame()

  return (
    <div className="gameover-modal">
      <div className="gameover-content">
        <h2>{won ? "Congratulations!" : "Game Over"}</h2>
        <p>
          {won
            ? `You guessed the word in ${attempts.length} ${attempts.length === 1 ? "try" : "tries"}!`
            : `The word was ${correctWord}.`}
        </p>

        <div className="stats-container">
          <div className="stat-box">
            <h3>Statistics</h3>
            <div className="stats-grid">
              <div className="stat-item">
                <div className="stat-number">{gameStats.played}</div>
                <div className="stat-label">Played</div>
              </div>
              <div className="stat-item">
                <div className="stat-number">{Math.round((gameStats.wins / gameStats.played) * 100) || 0}%</div>
                <div className="stat-label">Win %</div>
              </div>
              <div className="stat-item">
                <div className="stat-number">{gameStats.currentStreak}</div>
                <div className="stat-label">Current Streak</div>
              </div>
              <div className="stat-item">
                <div className="stat-number">{gameStats.maxStreak}</div>
                <div className="stat-label">Max Streak</div>
              </div>
            </div>
          </div>

          {won && (
            <div className="guess-distribution">
              <h3>Guess Distribution</h3>
              {Object.entries(gameStats.guessDistribution).map(([guess, count]) => (
                <div key={guess} className="distribution-row">
                  <div className="guess-number">{guess}</div>
                  <div className="guess-bar-container">
                    <div
                      className={`guess-bar ${Number(guess) === attempts.length ? "current" : ""}`}
                      style={{
                        width: `${Math.max(count * 5, count > 0 ? 10 : 0)}%`,
                      }}
                    >
                      {count}
                    </div>
                  </div>
                </div>
              ))}
            </div>
          )}
        </div>

        <button className="new-game-button" onClick={onNewGame}>
          Play Again
        </button>
      </div>
    </div>
  )
}

export default Gameover

