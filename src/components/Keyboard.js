const Keyboard = ({ onKeyPress, usedLetters }) => {
    const keyboardRows = [
      ["Q", "W", "E", "R", "T", "Y", "U", "I", "O", "P"],
      ["A", "S", "D", "F", "G", "H", "J", "K", "L"],
      ["ENTER", "Z", "X", "C", "V", "B", "N", "M", "BACKSPACE"],
    ]
  
    return (
      <div className="keyboard">
        {keyboardRows.map((row, rowIndex) => (
          <div key={rowIndex} className="keyboard-row">
            {row.map((key) => {
              const isSpecial = key === "ENTER" || key === "BACKSPACE"
              const status = usedLetters[key] || ""
  
              return (
                <button
                  key={key}
                  className={`keyboard-key ${isSpecial ? "keyboard-key-wide" : ""} ${status}`}
                  onClick={() => onKeyPress(key)}
                >
                  {key === "BACKSPACE" ? "⌫" : key}
                </button>
              )
            })}
          </div>
        ))}
      </div>
    )
  }
  
  export default Keyboard
  
  