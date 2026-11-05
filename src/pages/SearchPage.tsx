import { useState, useEffect, useRef } from 'react'
import { useSearchParams, useNavigate } from 'react-router-dom'
import { searchArticles } from '../lib/queries'
import type { Article } from '../types'
import ArticleCard from '../components/ArticleCard'
import LoadingState from '../components/LoadingState'

export default function SearchPage() {
  const [searchParams, setSearchParams] = useSearchParams()
  const navigate = useNavigate()
  const queryParam = searchParams.get('q') || ''
  const [inputValue, setInputValue] = useState(queryParam)
  const [results, setResults] = useState<Article[]>([])
  const [loading, setLoading] = useState(false)
  const [searched, setSearched] = useState(false)
  const inputRef = useRef<HTMLInputElement>(null)

  useEffect(() => {
    setInputValue(queryParam)
    if (queryParam.trim()) {
      setLoading(true)
      setSearched(true)
      searchArticles(queryParam, 50)
        .then((arts) => {
          setResults(arts)
          setLoading(false)
        })
        .catch(() => {
          setResults([])
          setLoading(false)
        })
    } else {
      setResults([])
      setSearched(false)
      setLoading(false)
    }
  }, [queryParam])

  useEffect(() => {
    inputRef.current?.focus()
  }, [])

  const handleSubmit = (e: React.FormEvent) => {
    e.preventDefault()
    const trimmed = inputValue.trim()
    if (trimmed) {
      setSearchParams({ q: trimmed })
    }
  }

  return (
    <div className="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 py-12">
      <div className="mb-10">
        <form onSubmit={handleSubmit}>
          <div className="relative">
            <input
              ref={inputRef}
              type="text"
              value={inputValue}
              onChange={(e) => setInputValue(e.target.value)}
              placeholder="Search articles…"
              className="w-full text-2xl sm:text-3xl font-serif text-neutral-800 bg-transparent border-b-2 border-neutral-300 focus:border-primary-700 outline-none py-3 pr-12 transition-colors placeholder:text-neutral-300"
            />
            <button
              type="submit"
              className="absolute right-0 top-1/2 -translate-y-1/2 text-neutral-400 hover:text-primary-700 transition-colors p-2"
              aria-label="Search"
            >
              <svg className="w-7 h-7" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M21 21l-6-6m2-5a7 7 0 11-14 0 7 7 0 0114 0z" />
              </svg>
            </button>
          </div>
        </form>
      </div>

      {loading ? (
        <LoadingState />
      ) : searched ? (
        results.length > 0 ? (
          <>
            <p className="text-sm text-neutral-500 mb-8">
              {results.length} result{results.length !== 1 ? 's' : ''} for "{queryParam}"
            </p>
            <div className="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 gap-8">
              {results.map((article) => (
                <ArticleCard key={article.id} article={article} />
              ))}
            </div>
          </>
        ) : (
          <div className="text-center py-20">
            <h2 className="text-2xl font-serif font-bold text-neutral-700 mb-3">No results found</h2>
            <p className="text-neutral-500 mb-6">We couldn't find any articles matching "{queryParam}".</p>
            <button
              onClick={() => navigate('/')}
              className="text-sm font-medium text-primary-700 hover:text-primary-800 transition-colors uppercase tracking-wider"
            >
              Back to Home →
            </button>
          </div>
        )
      ) : (
        <div className="text-center py-20">
          <p className="text-neutral-400 text-lg">Type something above to search all articles.</p>
        </div>
      )}
    </div>
  )
}
