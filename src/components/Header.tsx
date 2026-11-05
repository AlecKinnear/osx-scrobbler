import { useState, useEffect } from 'react'
import { Link } from 'react-router-dom'
import { fetchCategories } from '../lib/queries'
import type { Category } from '../types'
import Logo from './Logo'
import NewsletterSignup from './NewsletterSignup'

export default function Header() {
  const [categories, setCategories] = useState<Category[]>([])
  const [menuOpen, setMenuOpen] = useState(false)
  const [scrolled, setScrolled] = useState(false)

  useEffect(() => {
    fetchCategories().then(setCategories).catch(() => {})
    const onScroll = () => setScrolled(window.scrollY > 10)
    window.addEventListener('scroll', onScroll)
    return () => window.removeEventListener('scroll', onScroll)
  }, [])

  return (
    <header className={`sticky top-0 z-50 transition-all duration-300 ${scrolled ? 'bg-white shadow-md' : 'bg-white'}`}>
      <div className="border-b border-neutral-200">
        <div className="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
          <div className="flex items-center justify-between h-16">
            <Link to="/" className="group transition-transform group-hover:scale-[1.02] origin-left">
              <Logo />
            </Link>

            <nav className="hidden lg:flex items-center gap-6">
              {categories.map((cat) => (
                <Link
                  key={cat.id}
                  to={`/category/${cat.slug}`}
                  className="text-sm font-sans font-medium text-neutral-600 hover:text-primary-700 transition-colors"
                >
                  {cat.name}
                </Link>
              ))}
              <Link to="/about" className="text-sm font-sans font-medium text-neutral-600 hover:text-primary-700 transition-colors">
                About
              </Link>
            </nav>

            <button
              onClick={() => setMenuOpen(!menuOpen)}
              className="lg:hidden p-2 rounded-md text-neutral-600 hover:bg-neutral-100"
              aria-label="Toggle menu"
            >
              <svg className="w-6 h-6" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                {menuOpen ? (
                  <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M6 18L18 6M6 6l12 12" />
                ) : (
                  <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M4 6h16M4 12h16M4 18h16" />
                )}
              </svg>
            </button>
          </div>
        </div>
      </div>

      {menuOpen && (
        <div className="lg:hidden border-b border-neutral-200 bg-white">
          <nav className="px-4 py-3 space-y-2">
            {categories.map((cat) => (
              <Link
                key={cat.id}
                to={`/category/${cat.slug}`}
                onClick={() => setMenuOpen(false)}
                className="block py-2 text-sm font-medium text-neutral-600 hover:text-primary-700"
              >
                {cat.name}
              </Link>
            ))}
            <Link
              to="/about"
              onClick={() => setMenuOpen(false)}
              className="block py-2 text-sm font-medium text-neutral-600 hover:text-primary-700"
            >
              About
            </Link>
          </nav>
        </div>
      )}
    </header>
  )
}
