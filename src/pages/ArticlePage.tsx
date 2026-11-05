import { useState, useEffect } from 'react'
import { useParams, Link } from 'react-router-dom'
import { fetchArticleBySlug, fetchRecentArticles } from '../lib/queries'
import type { Article } from '../types'
import { formatDate, readingTime } from '../lib/utils'
import LoadingState from '../components/LoadingState'
import ErrorState from '../components/ErrorState'
import ArticleCard from '../components/ArticleCard'
import NewsletterSignup from '../components/NewsletterSignup'

export default function ArticlePage() {
  const { slug } = useParams<{ slug: string }>()
  const [article, setArticle] = useState<Article | null>(null)
  const [related, setRelated] = useState<Article[]>([])
  const [loading, setLoading] = useState(true)
  const [error, setError] = useState('')
  const [notFound, setNotFound] = useState(false)

  useEffect(() => {
    if (!slug) return
    setLoading(true)
    setError('')
    setNotFound(false)
    window.scrollTo(0, 0)

    fetchArticleBySlug(slug)
      .then((art) => {
        if (!art) {
          setNotFound(true)
          setLoading(false)
          return
        }
        setArticle(art)
        fetchRecentArticles(5, art.id)
          .then((recs) => setRelated(recs.filter((r) => r.category_id === art.category_id).slice(0, 3)))
          .catch(() => {})
        setLoading(false)
      })
      .catch((err) => {
        setError(err.message)
        setLoading(false)
      })
  }, [slug])

  if (loading) return <LoadingState />
  if (notFound)
    return (
      <div className="max-w-3xl mx-auto px-4 py-20 text-center">
        <h1 className="text-3xl font-serif font-bold text-neutral-800 mb-4">Article Not Found</h1>
        <p className="text-neutral-500 mb-8">The article you're looking for doesn't exist or has been removed.</p>
        <Link to="/" className="inline-block px-6 py-3 bg-primary-600 text-white rounded-lg hover:bg-primary-700 transition-colors">
          Back to Home
        </Link>
      </div>
    )
  if (error) return <ErrorState message={error} />
  if (!article) return <ErrorState message="Could not load article" />

  const shareUrl = typeof window !== 'undefined' ? window.location.href : ''
  const shareText = encodeURIComponent(article.title)

  return (
    <article>
      {/* Hero image */}
      {article.hero_image_url && (
        <div className="relative w-full h-[50vh] min-h-[400px] max-h-[600px] overflow-hidden">
          <img
            src={article.hero_image_url}
            alt={article.title}
            className="w-full h-full object-cover"
          />
          <div className="absolute inset-0 bg-gradient-to-t from-black/70 via-black/20 to-transparent" />
          <div className="absolute bottom-0 left-0 right-0">
            <div className="max-w-4xl mx-auto px-4 sm:px-6 lg:px-8 pb-10">
              {article.category && (
                <Link
                  to={`/category/${article.category.slug}`}
                  className="category-pill mb-4 inline-block"
                  style={{
                    color: article.category.color || '#1f4579',
                    backgroundColor: 'rgba(255,255,255,0.9)',
                  }}
                >
                  {article.category.name}
                </Link>
              )}
              <h1 className="text-2xl sm:text-4xl lg:text-5xl font-serif font-bold text-white leading-tight max-w-3xl">
                {article.title}
              </h1>
            </div>
          </div>
        </div>
      )}

      {/* Article body */}
      <div className="max-w-4xl mx-auto px-4 sm:px-6 lg:px-8 py-12">
        {!article.hero_image_url && (
          <div className="mb-8">
            {article.category && (
              <Link
                to={`/category/${article.category.slug}`}
                className="category-pill mb-4 inline-block"
                style={{
                  color: article.category.color || '#1f4579',
                  backgroundColor: `${article.category.color}20`,
                }}
              >
                {article.category.name}
              </Link>
            )}
            <h1 className="text-3xl sm:text-4xl lg:text-5xl font-serif font-bold text-neutral-900 leading-tight">
              {article.title}
            </h1>
          </div>
        )}

        {/* Meta bar */}
        <div className="flex flex-wrap items-center justify-between gap-4 pb-6 mb-8 border-b border-neutral-200">
          <div className="flex items-center gap-3">
            {article.author && (
              <Link to={`/author/${article.author.slug}`} className="flex items-center gap-3 group">
                <div className="w-10 h-10 rounded-full bg-primary-100 flex items-center justify-center text-primary-700 font-serif font-bold text-sm">
                  {article.author.name.split(' ').map((n) => n[0]).slice(0, 2).join('')}
                </div>
                <div>
                  <div className="text-sm font-medium text-neutral-800 group-hover:text-primary-700 transition-colors">
                    {article.author.name}
                  </div>
                  <div className="text-xs text-neutral-400">
                    {formatDate(article.published_at)} · {readingTime(article.body)}
                  </div>
                </div>
              </Link>
            )}
          </div>

          {/* Share buttons */}
          <div className="flex items-center gap-2">
            <span className="text-xs text-neutral-400 mr-1">Share:</span>
            <a
              href={`https://bsky.app/intent/compose?text=${shareText}%20${encodeURIComponent(shareUrl)}`}
              target="_blank"
              rel="noopener noreferrer"
              className="w-8 h-8 flex items-center justify-center rounded-full bg-neutral-100 hover:bg-primary-600 hover:text-white text-neutral-500 transition-all"
              aria-label="Share on Bluesky"
            >
              <svg className="w-4 h-4" fill="currentColor" viewBox="0 0 24 24"><path d="M12 2L8 8H4l4 6-4 6h4l4-6 4 6h4l-4-6 4-6h-4l-4-6z" /></svg>
            </a>
            <a
              href={`https://www.facebook.com/sharer/sharer.php?u=${encodeURIComponent(shareUrl)}`}
              target="_blank"
              rel="noopener noreferrer"
              className="w-8 h-8 flex items-center justify-center rounded-full bg-neutral-100 hover:bg-primary-600 hover:text-white text-neutral-500 transition-all"
              aria-label="Share on Facebook"
            >
              <svg className="w-4 h-4" fill="currentColor" viewBox="0 0 24 24"><path d="M24 12.07C24 5.4 18.63 0 12 0S0 5.4 0 12.07C0 18.1 4.39 23.1 10.13 24v-8.44H7.08v-3.49h3.05V9.41c0-3.02 1.79-4.69 4.53-4.69 1.31 0 2.69.24 2.69.24v2.97h-1.52c-1.49 0-1.96.93-1.96 1.89v2.25h3.33l-.53 3.49h-2.8V24C19.61 23.1 24 18.1 24 12.07z" /></svg>
            </a>
            <a
              href={`https://www.reddit.com/submit?url=${encodeURIComponent(shareUrl)}&title=${shareText}`}
              target="_blank"
              rel="noopener noreferrer"
              className="w-8 h-8 flex items-center justify-center rounded-full bg-neutral-100 hover:bg-primary-600 hover:text-white text-neutral-500 transition-all"
              aria-label="Share on Reddit"
            >
              <svg className="w-4 h-4" fill="currentColor" viewBox="0 0 24 24"><path d="M24 12c0-1.7-1.4-3-3-3-.8 0-1.5.3-2 .8-2-1.3-4.7-2.1-7.6-2.2l1.3-6L16.5 3c.1 1.2 1.1 2.2 2.3 2.2 1.3 0 2.3-1 2.3-2.3s-1-2.3-2.3-2.3c-.9 0-1.7.5-2 1.2l-6.2-1.2c-.2 0-.4.1-.4.3L8.7 7.5C5.8 7.7 3 8.5 1 9.8.5 9.3-.2 9-1 9c-1.6 0-3 1.3-3 3 0 1.1.6 2.1 1.6 2.6-.1.3-.1.6-.1.9 0 3.1 3.6 5.6 8 5.6s8-2.5 8-5.6c0-.3 0-.6-.1-.9 1-.5 1.6-1.5 1.6-2.6z" /></svg>
            </a>
            <button
              onClick={() => { navigator.clipboard.writeText(shareUrl) }}
              className="w-8 h-8 flex items-center justify-center rounded-full bg-neutral-100 hover:bg-primary-600 hover:text-white text-neutral-500 transition-all"
              aria-label="Copy link"
            >
              <svg className="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M13.828 10.172a4 4 0 00-5.656 0l-4 4a4 4 0 105.656 5.656l1.102-1.101m-.758-4.899a4 4 0 005.656 0l4-4a4 4 0 00-5.656-5.656l-1.1 1.1" /></svg>
            </button>
          </div>
        </div>

        {/* Hero image caption */}
        {article.hero_image_url && article.hero_image_caption && (
          <p className="text-xs text-neutral-400 italic mb-8 -mt-4">{article.hero_image_caption}</p>
        )}

        {/* Excerpt */}
        {article.excerpt && (
          <p className="text-xl font-serif text-neutral-600 leading-relaxed mb-8 italic border-l-4 border-primary-300 pl-6">
            {article.excerpt}
          </p>
        )}

        {/* Body */}
        <div className="article-body" dangerouslySetInnerHTML={{ __html: article.body }} />

        {/* Tags */}
        {article.tags.length > 0 && (
          <div className="mt-10 pt-6 border-t border-neutral-200">
            <div className="flex flex-wrap gap-2">
              {article.tags.map((tag) => (
                <span key={tag} className="px-3 py-1 bg-neutral-100 text-neutral-500 text-xs rounded-full">
                  {tag}
                </span>
              ))}
            </div>
          </div>
        )}

        {/* Author bio */}
        {article.author && (
          <div className="mt-12 p-6 bg-neutral-50 rounded-2xl border border-neutral-100">
            <div className="flex items-start gap-4">
              <div className="w-14 h-14 rounded-full bg-primary-100 flex items-center justify-center text-primary-700 font-serif font-bold text-lg flex-shrink-0">
                {article.author.name.split(' ').map((n) => n[0]).slice(0, 2).join('')}
              </div>
              <div>
                <Link to={`/author/${article.author.slug}`} className="text-lg font-serif font-bold text-neutral-800 hover:text-primary-700 transition-colors">
                  {article.author.name}
                </Link>
                {article.author.bio && (
                  <p className="text-sm text-neutral-600 mt-1 leading-relaxed">{article.author.bio}</p>
                )}
                <div className="flex items-center gap-3 mt-3">
                  {article.author.bluesky_url && (
                    <a href={article.author.bluesky_url} target="_blank" rel="noopener noreferrer" className="text-xs text-primary-600 hover:text-primary-700 font-medium">
                      Bluesky
                    </a>
                  )}
                  {article.author.facebook_url && (
                    <a href={article.author.facebook_url} target="_blank" rel="noopener noreferrer" className="text-xs text-primary-600 hover:text-primary-700 font-medium">
                      Facebook
                    </a>
                  )}
                </div>
              </div>
            </div>
          </div>
        )}

        {/* Newsletter */}
        <div className="mt-12 p-8 bg-ink-900 rounded-2xl text-white text-center">
          <h3 className="text-xl font-serif font-bold mb-2">Join Our Newsletter</h3>
          <p className="text-sm text-neutral-300 mb-4">Sharp analysis delivered to your inbox every day.</p>
          <div className="max-w-md mx-auto">
            <NewsletterSignup dark />
          </div>
        </div>
      </div>

      {/* Related articles */}
      {related.length > 0 && (
        <section className="bg-neutral-50 py-12 border-t border-neutral-200">
          <div className="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
            <h2 className="text-sm font-sans font-bold uppercase tracking-widest text-neutral-400 mb-6">
              More from {article.category?.name}
            </h2>
            <div className="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 gap-8">
              {related.map((rel) => (
                <ArticleCard key={rel.id} article={rel} />
              ))}
            </div>
          </div>
        </section>
      )}
    </article>
  )
}
