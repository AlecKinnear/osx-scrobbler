import { useState, useEffect, useRef } from 'react'
import { Link, useNavigate } from 'react-router-dom'
import { fetchCategories, searchArticles } from '../lib/queries'
import type { Category, Article } from '../types'
import Logo from './Logo'
import { formatDateShort } from '../lib/utils'

export default function Header() {
  const [categories, setCategories] = useState<Category[]>([])
  const [menuOpen, setMenuOpen] = useState(false)
  const [scrolled, setScrolled] = useState(false)
  const [searchOpen, setSearchOpen] = useState(false)
  const [searchQuery, setSearchQuery] = useState('')
  const [searchResults, setSearchResults] = useState<Article[]>([])
  const [searchLoading, setSearchLoading] = useState(false)
  const searchInputRef = useRef<HTMLInputElement>(null)
  const searchContainerRef = useRef<HTMLDivElement>(null)
  const debounceRef = useRef<ReturnType<typeof setTimeout> | null>(null)
  const navigate = useNavigate()

  useEffect(() => {
    fetchCategories().then(setCategories).catch(() => {})
    const onScroll = () => setScrolled(window.scrollY > 10)
    window.addEventListener('scroll', onScroll)
    return () => window.removeEventListener('scroll', onScroll)
  }, [])

  useEffect(() => {
    if (searchOpen) {
      searchInputRef.current?.focus()
    } else {
      setSearchQuery('')
      setSearchResults([])
    }
  }, [searchOpen])

  useEffect(() => {
    const trimmed = searchQuery.trim()
    if (!trimmed) {
      setSearchResults([])
      setSearchLoading(false)
      return
    }
    setSearchLoading(true)
    if (debounceRef.current) clearTimeout(debounceRef.current)
    debounceRef.current = setTimeout(() => {
      searchArticles(trimmed, 6)
        .then((arts) => {
          setSearchResults(arts)
          setSearchLoading(false)
        })
        .catch(() => {
          setSearchResults([])
          setSearchLoading(false)
        })
    }, 300)
    return () => {
      if (debounceRef.current) clearTimeout(debounceRef.current)
    }
  }, [searchQuery])

  const closeSearch = () => setSearchOpen(false)

  const handleSubmitSearch = (e: React.FormEvent) => {
    e.preventDefault()
    const trimmed = searchQuery.trim()
    if (trimmed) {
      closeSearch()
      navigate(`/search?q=${encodeURIComponent(trimmed)}`)
    }
  }

  const closeSearchOnOutside = (e: React.MouseEvent) => {
    if (searchContainerRef.current && !searchContainerRef.current.contains(e.target as Node)) {
      closeSearch()
    }
  }

  const SearchIcon = ({ className = 'w-5 h-5' }: { className?: string }) => (
    <svg className={className} fill="none" stroke="currentColor" viewBox="0 0 24 24">
      <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M21 21l-6-6m2-5a7 7 0 11-14 0 7 7 0 0114 0z" />
    </svg>
  )

  const CloseIcon = ({ className = 'w-5 h-5' }: { className?: string }) => (
    <svg className={className} fill="none" stroke="currentColor" viewBox="0 0 24 24">
      <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M6 18L18 6M6 6l12 12" />
    </svg>
  )

  return (
    <header className={`sticky top-0 z-50 transition-all duration-300 ${scrolled ? 'bg-white shadow-md' : 'bg-white'}`}>
      <div>
        <div className="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
          <div className="flex items-center justify-between h-20 gap-8">
            {!searchOpen && (
              <Link to="/" className="group transition-transform group-hover:scale-[1.02] origin-left flex-shrink-0">
                <Logo />
              </Link>
            )}

            {/* Nav or search on the right */}
            {!searchOpen && (
              <>
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
                  <button
                    onClick={() => setSearchOpen(true)}
                    className="text-neutral-600 hover:text-primary-700 transition-colors"
                    aria-label="Search"
                  >
                    <SearchIcon />
                  </button>
                </nav>

                <div className="flex items-center gap-2 lg:hidden">
                  <button
                    onClick={() => setSearchOpen(true)}
                    className="p-2 rounded-md text-neutral-600 hover:bg-neutral-100"
                    aria-label="Search"
                  >
                    <SearchIcon className="w-5 h-5" />
                  </button>
                  <button
                    onClick={() => setMenuOpen(!menuOpen)}
                    className="p-2 rounded-md text-neutral-600 hover:bg-neutral-100"
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
              </>
            )}

            {/* Search mode: appears in the nav space, not over the logo */}
            {searchOpen && (
              <div ref={searchContainerRef} className="w-[640px] max-w-full flex items-center gap-3">
                <form onSubmit={handleSubmitSearch} className="relative flex-1">
                  <input
                    ref={searchInputRef}
                    type="text"
                    value={searchQuery}
                    onChange={(e) => setSearchQuery(e.target.value)}
                    placeholder="Search articles…"
                    className="w-full text-sm font-sans font-medium text-neutral-600 bg-transparent border-b border-neutral-300 focus:border-neutral-400 outline-none py-0 pr-8 placeholder:text-neutral-400 placeholder:font-normal transition-colors text-left"
                  />
                  {searchQuery && (
                    <button
                      type="button"
                      onClick={() => {
                        setSearchQuery('')
                        searchInputRef.current?.focus()
                      }}
                      className="absolute right-0 top-1/2 -translate-y-1/2 text-neutral-300 hover:text-neutral-500 transition-colors p-1"
                      aria-label="Clear"
                    >
                      <CloseIcon className="w-4 h-4" />
                    </button>
                  )}

                  {/* Live results dropdown */}
                  {(searchResults.length > 0 || (searchLoading && searchQuery.trim())) && (
                    <div className="absolute top-full left-0 right-0 mt-1 bg-white rounded-xl shadow-2xl border border-neutral-100 overflow-hidden z-50">
                      {searchLoading && (
                        <div className="px-5 py-4 text-sm text-neutral-400">Searching…</div>
                      )}
                      {!searchLoading && searchResults.length > 0 && (
                        <>
                          {searchResults.map((article) => (
                            <Link
                              key={article.id}
                              to={`/article/${article.slug}`}
                              onClick={closeSearch}
                              className="block px-5 py-3 hover:bg-neutral-50 transition-colors border-b border-neutral-50 last:border-b-0"
                            >
                              <div className="flex items-start gap-3">
                                {article.hero_image_url && (
                                  <img
                                    src={article.hero_image_url}
                                    alt={article.title}
                                    className="w-14 h-14 object-cover rounded-md flex-shrink-0"
                                    loading="lazy"
                                  />
                                )}
                                <div className="min-w-0 flex-1">
                                  {article.category && (
                                    <span
                                      className="text-[10px] font-sans font-semibold uppercase tracking-wider"
                                      style={{ color: article.category.color || '#1f4579' }}
                                    >
                                      {article.category.name}
                                    </span>
                                  )}
                                  <h4 className="text-sm font-serif font-bold text-neutral-800 leading-snug line-clamp-2 mt-0.5">
                                    {article.title}
                                  </h4>
                                  <span className="text-xs text-neutral-400 mt-0.5 block">{formatDateShort(article.published_at)}</span>
                                </div>
                              </div>
                            </Link>
                          ))}
                          <button
                            onClick={handleSubmitSearch as unknown as () => void}
                            className="block w-full text-center py-3 text-xs font-sans font-medium uppercase tracking-wider text-primary-700 hover:bg-primary-50 transition-colors"
                          >
                            See all results →
                          </button>
                        </>
                      )}
                    </div>
                  )}
                </form>
                <button
                  onClick={closeSearch}
                  className="flex-shrink-0 p-2 rounded-md text-neutral-500 hover:bg-neutral-100 transition-colors"
                  aria-label="Close search"
                >
                  <CloseIcon className="w-5 h-5" />
                </button>
              </div>
            )}
          </div>
        </div>
      </div>

      {/* Mobile menu (only in normal mode) */}
      {!searchOpen && menuOpen && (
        <div className="lg:hidden bg-white">
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

      {/* Click-outside overlay for search */}
      {searchOpen && <div className="fixed inset-0 z-40" onClick={closeSearchOnOutside} />}
    </header>
  )
}
