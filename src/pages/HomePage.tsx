import { useState, useEffect } from 'react'
import { Link } from 'react-router-dom'
import { fetchFeaturedArticle, fetchArticlesGroupedByCategory } from '../lib/queries'
import type { Article, Category } from '../types'
import ArticleHero from '../components/ArticleHero'
import ArticleCard from '../components/ArticleCard'
import LoadingState from '../components/LoadingState'
import ErrorState from '../components/ErrorState'

export default function HomePage() {
  const [featured, setFeatured] = useState<Article | null>(null)
  const [groups, setGroups] = useState<{ category: Category; articles: Article[] }[]>([])
  const [loading, setLoading] = useState(true)
  const [error, setError] = useState('')

  useEffect(() => {
    fetchFeaturedArticle()
      .then((feat) => {
        setFeatured(feat)
        return fetchArticlesGroupedByCategory(3, feat?.id)
      })
      .then((g) => {
        setGroups(g)
        setLoading(false)
      })
      .catch((err) => {
        setError(err.message)
        setLoading(false)
      })
  }, [])

  if (loading) return <LoadingState />
  if (error) return <ErrorState message={error} />

  return (
    <div>
      {/* Hero + Support box */}
      <section className="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 pt-8 pb-12">
        <div className="grid grid-cols-1 lg:grid-cols-3 gap-8">
          <div className="lg:col-span-2">
            {featured && <ArticleHero article={featured} />}
          </div>
          <div className="lg:col-span-1">
            <div className="bg-gradient-to-br from-primary-700 to-primary-900 rounded-2xl p-6 text-white h-full flex flex-col justify-center">
              <h3 className="text-lg font-serif font-bold mb-2">Support Informed Comment</h3>
              <p className="text-sm text-primary-100 leading-relaxed mb-4">
                We can't provide our coverage without your help. Your support keeps independent journalism alive.
              </p>
              <div className="mb-4">
                <div className="flex justify-between text-xs mb-1.5">
                  <span className="text-primary-200">$22,622 raised</span>
                  <span className="text-primary-200">Goal: $30,000</span>
                </div>
                <div className="h-2 bg-primary-900 rounded-full overflow-hidden">
                  <div className="h-full bg-accent-400 rounded-full" style={{ width: '75.4%' }} />
                </div>
              </div>
              <button className="w-full py-2.5 bg-accent-500 hover:bg-accent-600 text-white text-sm font-medium rounded-lg transition-colors">
                Donate Now
              </button>
            </div>
          </div>
        </div>
      </section>

      {/* Category sections — 3 across grid, ordered by latest story per category */}
      <section className="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 pb-12">
        <div className="space-y-12">
          {groups.map(({ category, articles }) => (
            <div key={category.id}>
              <div className="flex items-center justify-between mb-4">
                <Link
                  to={`/category/${category.slug}`}
                  className="text-sm font-sans font-bold uppercase tracking-wider hover:opacity-70 transition-opacity"
                  style={{ color: category.color || '#1f4579' }}
                >
                  {category.name}
                </Link>
                <Link
                  to={`/category/${category.slug}`}
                  className="text-xs font-sans font-medium text-neutral-400 hover:text-primary-700 transition-colors uppercase tracking-wider"
                >
                  View All →
                </Link>
              </div>
              <div className="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 gap-8">
                {articles.map((article) => (
                  <ArticleCard key={article.id} article={article} />
                ))}
              </div>
            </div>
          ))}
        </div>
      </section>
    </div>
  )
}
