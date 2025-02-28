"use client"

import { useState } from "react"
import { useRouter } from "next/router"
import { useGame } from "../context/GameContext"

const Login = () => {
  const [username, setUsername] = useState("")
  const [password, setPassword] = useState("")
  const [error, setError] = useState("")
  const [isLoading, setIsLoading] = useState(false)
  const { login } = useGame()
  const router = useRouter()

  // Simulated loading for futuristic effect
  const handleLogin = (e) => {
    e.preventDefault()
    if (username.trim() && password.trim()) {
      setIsLoading(true)
      // Simulate API call
      setTimeout(() => {
        login(username)
        router.push("/game")
      }, 800)
    } else {
      setError("Please enter both username and password")
    }
  }

  return (
    <div className="login-container">
      <form onSubmit={handleLogin} className="login-form">
        <h2>Login to Play</h2>
        {error && <div className="error-message">{error}</div>}
        <div className="form-group">
          <label htmlFor="username">Username</label>
          <input
            id="username"
            type="text"
            placeholder="Enter your username"
            value={username}
            onChange={(e) => setUsername(e.target.value)}
            className="form-input"
            disabled={isLoading}
          />
        </div>
        <div className="form-group">
          <label htmlFor="password">Password</label>
          <input
            id="password"
            type="password"
            placeholder="Enter your password"
            value={password}
            onChange={(e) => setPassword(e.target.value)}
            className="form-input"
            disabled={isLoading}
          />
        </div>
        <button type="submit" className="login-button" disabled={isLoading}>
          {isLoading ? "Logging in..." : "Login"}
        </button>
      </form>
    </div>
  )
}

export default Login

