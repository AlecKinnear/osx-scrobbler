import { useState, useEffect } from 'react'
import { useParams, Link } from 'react-router-dom'
import { fetchAuthorBySlug, fetchArticlesByAuthor } from '../lib/queries'
import type { Article, Author } from '../types'
import ArticleCard from '../components/ArticleCard'
import LoadingState from '../components/LoadingState'
import ErrorState from '../components/ErrorState'

export default function AuthorPage() {
  const { slug } = useParams<{ slug: string }>()
  const [author, setAuthor] = useState<Author | null>(null)
  const [articles, setArticles] = useState<Article[]>([])
  const [loading, setLoading] = useState(true)
  const [error, setError] = useState('')

  useEffect(() => {
    if (!slug) return
    setLoading(true)
    window.scrollTo(0, 0)

    Promise.all([
      fetchAuthorBySlug(slug),
      fetchArticlesByAuthor(slug),
    ])
      .then(([auth, arts]) => {
        setAuthor(auth)
        setArticles(arts)
        setLoading(false)
      })
      .catch((err) => {
        setError(err.message)
        setLoading(false)
      })
  }, [slug])

  if (loading) return <LoadingState />
  if (error) return <ErrorState message={error} />
  if (!author)
    return (
      <div className="max-w-3xl mx-auto px-4 py-20 text-center">
        <h1 className="text-3xl font-serif font-bold text-neutral-800 mb-4">Author Not Found</h1>
        <p className="text-neutral-500">This author doesn't exist.</p>
      </div>
    )

  return (
    <div>
      <div className="bg-neutral-50 py-16 border-b border-neutral-200">
        <div className="max-w-4xl mx-auto px-4 sm:px-6 lg:px-8">
          <div className="flex items-start gap-6">
            <div className="w-20 h-20 rounded-full bg-primary-100 flex items-center justify-center text-primary-700 font-serif font-bold text-2xl flex-shrink-0">
              {author.name.split(' ').map((n) => n[0]).slice(0, 2).join('')}
            </div>
            <div>
              <h1 className="text-3xl sm:text-4xl font-serif font-bold text-neutral-900">{author.name}</h1>
              {author.bio && (
                <p className="text-neutral-600 mt-3 leading-relaxed max-w-3xl">{author.bio}</p>
              )}
              <div className="flex items-center gap-3 mt-4">
                {author.bluesky_url && (
                  <a href={author.bluesky_url} target="_blank" rel="noopener noreferrer" className="text-sm text-primary-600 hover:text-primary-700 font-medium">
                    Bluesky
                  </a>
                )}
                {author.facebook_url && (
                  <a href={author.facebook_url} target="_blank" rel="noopener noreferrer" className="text-sm text-primary-600 hover:text-primary-700 font-medium">
                    Facebook
                  </a>
                )}
              </div>
            </div>
          </div>
        </div>
      </div>

      <div className="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 py-12">
        <h2 className="text-sm font-sans font-bold uppercase tracking-widest text-neutral-400 mb-8 pb-3 border-b border-neutral-200">
          Articles by {author.name}
        </h2>
        {articles.length === 0 ? (
          <p className="text-center text-neutral-500 py-20">No articles yet.</p>
        ) : (
          <div className="space-y-8">
            {articles.map((article) => (
              <ArticleCard key={article.id} article={article} variant="horizontal" />
            ))}
          </div>
        )}
      </div>
    </div>
  )
}
