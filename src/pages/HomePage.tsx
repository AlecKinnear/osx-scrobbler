import { useState, useEffect } from 'react'
import { fetchFeaturedArticle, fetchRecentArticles } from '../lib/queries'
import type { Article } from '../types'
import ArticleHero from '../components/ArticleHero'
import ArticleCard from '../components/ArticleCard'
import NewsletterSignup from '../components/NewsletterSignup'
import LoadingState from '../components/LoadingState'
import ErrorState from '../components/ErrorState'
import { formatDateShort } from '../lib/utils'

export default function HomePage() {
  const [featured, setFeatured] = useState<Article | null>(null)
  const [articles, setArticles] = useState<Article[]>([])
  const [loading, setLoading] = useState(true)
  const [error, setError] = useState('')

  useEffect(() => {
    Promise.all([
      fetchFeaturedArticle(),
      fetchRecentArticles(14),
    ])
      .then(([feat, all]) => {
        setFeatured(feat)
        setArticles(feat ? all.filter((a) => a.id !== feat.id) : all)
        setLoading(false)
      })
      .catch((err) => {
        setError(err.message)
        setLoading(false)
      })
  }, [])

  if (loading) return <LoadingState />
  if (error) return <ErrorState message={error} />

  const sidebar = articles.slice(0, 4)
  const mainGrid = articles.slice(4, 10)

  return (
    <div>
      {/* Hero section */}
      <section className="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 pt-8 pb-12">
        {featured && <ArticleHero article={featured} />}
      </section>

      {/* Main content + sidebar */}
      <section className="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 pb-12">
        <div className="grid grid-cols-1 lg:grid-cols-3 gap-12">
          {/* Main column */}
          <div className="lg:col-span-2 space-y-10">
            <div>
              <h2 className="text-sm font-sans font-bold uppercase tracking-widest text-neutral-400 mb-6 pb-3 border-b border-neutral-200">
                More Stories
              </h2>
              <div className="space-y-8">
                {articles.slice(0, 4).map((article) => (
                  <ArticleCard key={article.id} article={article} variant="horizontal" />
                ))}
              </div>
            </div>

            {mainGrid.length > 0 && (
              <div>
                <h2 className="text-sm font-sans font-bold uppercase tracking-widest text-neutral-400 mb-6 pb-3 border-b border-neutral-200">
                  Latest
                </h2>
                <div className="grid grid-cols-1 sm:grid-cols-2 gap-8">
                  {mainGrid.map((article) => (
                    <ArticleCard key={article.id} article={article} />
                  ))}
                </div>
              </div>
            )}
          </div>

          {/* Sidebar */}
          <aside className="space-y-8">
            {/* Fundraiser widget */}
            <div className="bg-gradient-to-br from-primary-700 to-primary-900 rounded-2xl p-6 text-white">
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

            {/* Latest in sidebar */}
            <div className="bg-neutral-50 rounded-2xl p-6 border border-neutral-100">
              <h3 className="text-sm font-sans font-bold uppercase tracking-widest text-neutral-400 mb-4 pb-3 border-b border-neutral-200">
                Latest
              </h3>
              <div className="space-y-4">
                {sidebar.map((article) => (
                  <ArticleCard key={article.id} article={article} variant="compact" />
                ))}
              </div>
            </div>

            {/* Newsletter */}
            <div className="bg-ink-900 rounded-2xl p-6 text-white">
              <h3 className="text-lg font-serif font-bold mb-1">Newsletter</h3>
              <p className="text-sm text-neutral-300 mb-4">
                Sharp analysis delivered to your inbox every day.
              </p>
              <NewsletterSignup dark />
            </div>
          </aside>
        </div>
      </section>
    </div>
  )
}
