import { forwardRef } from "react"

const WordleGrid = forwardRef(({ attempts, currentAttempt, correctWord, shake }, ref) => {
  const getColorClass = (letter, index, attempt, attemptIndex) => {
    if (!letter) return ""

    // If this is a submitted attempt
    if (attemptIndex < attempts.length) {
      if (correctWord[index] === letter) {
        return "correct"
      } else if (correctWord.includes(letter)) {
        // Count occurrences of the letter in the correct word
        const correctOccurrences = correctWord.split("").filter((l) => l === letter).length

        // Count occurrences of the correctly placed letter in the attempt
        let correctPositions = 0
        for (let i = 0; i < attempt.length; i++) {
          if (attempt[i] === letter && correctWord[i] === letter) {
            correctPositions++
          }
        }

        // Count the number of 'present' cells for this letter before this position
        let presentBefore = 0
        for (let i = 0; i < index; i++) {
          if (attempt[i] === letter && correctWord[i] !== letter) {
            presentBefore++
          }
        }

        if (presentBefore + correctPositions < correctOccurrences) {
          return "present"
        } else {
          return "absent"
        }
      } else {
        return "absent"
      }
    }
    return ""
  }

  return (
    <div className="wordle-grid" ref={ref}>
      {Array.from({ length: 6 }).map((_, rowIndex) => {
        const attempt = attempts[rowIndex] || ""
        const isCurrentRow = rowIndex === attempts.length
        const rowClass = isCurrentRow && shake ? "wordle-row shake" : "wordle-row"

        return (
          <div key={rowIndex} className={rowClass}>
            {Array.from({ length: 5 }).map((_, colIndex) => {
              const letter = attempts[rowIndex]?.[colIndex] || (isCurrentRow ? currentAttempt[colIndex] : "")
              const colorClass = getColorClass(letter, colIndex, attempt, rowIndex)
              const cellStatus = rowIndex < attempts.length ? "revealed" : isCurrentRow && letter ? "filled" : ""

              return (
                <div
                  key={colIndex}
                  className={`wordle-cell ${colorClass} ${cellStatus}`}
                  style={{
                    transitionDelay: `${colIndex * 100}ms`,
                    animationDelay: `${colIndex * 100}ms`,
                  }}
                >
                  {letter}
                </div>
              )
            })}
          </div>
        )
      })}
    </div>
  )
})

WordleGrid.displayName = "WordleGrid"
export default WordleGrid

