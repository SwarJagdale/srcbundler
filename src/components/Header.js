"use client"

import { useRouter } from "next/router"
import { useGame } from "../context/GameContext"

const Header = () => {
  const { user, logout } = useGame()
  const router = useRouter()

  const handleLogout = () => {
    logout()
    router.push("/")
  }

  return (
    <header className="game-header">
      <div className="header-left">
        <button className="icon-button" aria-label="Help">
          <svg
            xmlns="http://www.w3.org/2000/svg"
            viewBox="0 0 24 24"
            fill="none"
            stroke="currentColor"
            strokeWidth="2"
            strokeLinecap="round"
            strokeLinejoin="round"
          >
            <circle cx="12" cy="12" r="10"></circle>
            <path d="M9.09 9a3 3 0 0 1 5.83 1c0 2-3 3-3 3"></path>
            <line x1="12" y1="17" x2="12.01" y2="17"></line>
          </svg>
        </button>
      </div>

      <h1 className="game-title">WORDLE</h1>

      <div className="header-right">
        <div className="user-info">
          <span>Hi, {user?.username}</span>
          <button className="logout-button" onClick={handleLogout}>
            Logout
          </button>
        </div>
      </div>
    </header>
  )
}

export default Header

