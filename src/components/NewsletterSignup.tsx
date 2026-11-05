import { useState } from 'react'
import { subscribeNewsletter } from '../lib/queries'

export default function NewsletterSignup({ dark = false }: { dark?: boolean }) {
  const [email, setEmail] = useState('')
  const [status, setStatus] = useState<'idle' | 'loading' | 'success' | 'error'>('idle')
  const [message, setMessage] = useState('')

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault()
    if (!email) return
    setStatus('loading')
    const { error } = await subscribeNewsletter(email)
    if (error) {
      setStatus('error')
      setMessage(error)
    } else {
      setStatus('success')
      setMessage('Thank you for subscribing!')
      setEmail('')
    }
  }

  const inputClass = dark
    ? 'w-full px-4 py-2.5 bg-neutral-800 border border-neutral-600 text-white placeholder-neutral-500 rounded-lg focus:outline-none focus:border-primary-400 text-sm'
    : 'w-full px-4 py-2.5 bg-white border border-neutral-300 text-neutral-800 placeholder-neutral-400 rounded-lg focus:outline-none focus:border-primary-500 text-sm'

  return (
    <form onSubmit={handleSubmit} className="space-y-2">
      <div className="flex gap-2">
        <input
          type="email"
          value={email}
          onChange={(e) => setEmail(e.target.value)}
          placeholder="your@email.com"
          className={inputClass}
          disabled={status === 'loading'}
          required
        />
        <button
          type="submit"
          disabled={status === 'loading'}
          className="px-4 py-2.5 bg-primary-600 text-white text-sm font-medium rounded-lg hover:bg-primary-700 transition-colors disabled:opacity-50 whitespace-nowrap"
        >
          {status === 'loading' ? '...' : 'Subscribe'}
        </button>
      </div>
      {status === 'success' && (
        <p className={`text-xs ${dark ? 'text-primary-300' : 'text-primary-700'}`}>{message}</p>
      )}
      {status === 'error' && (
        <p className={`text-xs ${dark ? 'text-accent-300' : 'text-error-600'}`}>{message}</p>
      )}
    </form>
  )
}
