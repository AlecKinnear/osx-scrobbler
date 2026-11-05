import { useState, useEffect } from 'react'
import { useParams } from 'react-router-dom'
import { fetchCategoryBySlug, fetchArticlesByCategory } from '../lib/queries'
import type { Article, Category } from '../types'
import ArticleCard from '../components/ArticleCard'
import LoadingState from '../components/LoadingState'
import ErrorState from '../components/ErrorState'

export default function CategoryPage() {
  const { slug } = useParams<{ slug: string }>()
  const [category, setCategory] = useState<Category | null>(null)
  const [articles, setArticles] = useState<Article[]>([])
  const [loading, setLoading] = useState(true)
  const [error, setError] = useState('')

  useEffect(() => {
    if (!slug) return
    setLoading(true)
    window.scrollTo(0, 0)

    Promise.all([
      fetchCategoryBySlug(slug),
      fetchArticlesByCategory(slug),
    ])
      .then(([cat, arts]) => {
        setCategory(cat)
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
  if (!category)
    return (
      <div className="max-w-3xl mx-auto px-4 py-20 text-center">
        <h1 className="text-3xl font-serif font-bold text-neutral-800 mb-4">Category Not Found</h1>
        <p className="text-neutral-500">This section doesn't exist.</p>
      </div>
    )

  return (
    <div>
      <div
        className="py-16"
        style={{
          backgroundColor: category.color ? `${category.color}10` : '#f5f8f6',
          borderBottom: `3px solid ${category.color || '#38704f'}`,
        }}
      >
        <div className="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
          <h1 className="text-3xl sm:text-4xl lg:text-5xl font-serif font-bold" style={{ color: category.color || '#38704f' }}>
            {category.name}
          </h1>
          {category.description && (
            <p className="text-lg text-neutral-600 mt-3 max-w-2xl leading-relaxed">{category.description}</p>
          )}
          <p className="text-sm text-neutral-400 mt-4">{articles.length} article{articles.length !== 1 ? 's' : ''}</p>
        </div>
      </div>

      <div className="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 py-12">
        {articles.length === 0 ? (
          <p className="text-center text-neutral-500 py-20">No articles in this section yet.</p>
        ) : (
          <>
            {articles[0] && (
              <div className="mb-12">
                <ArticleCard article={articles[0]} variant="horizontal" />
              </div>
            )}
            <div className="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 gap-8">
              {articles.slice(1).map((article) => (
                <ArticleCard key={article.id} article={article} />
              ))}
            </div>
          </>
        )}
      </div>
    </div>
  )
}
