import { Link } from 'react-router-dom'
import type { Article } from '../types'
import { formatDate, readingTime } from '../lib/utils'
import { categoryStyle } from './ArticleCard'

export default function ArticleHero({ article }: { article: Article }) {
  return (
    <article className="relative">
      <Link to={`/article/${article.slug}`} className="group block">
        <div className="relative aspect-[21/9] sm:aspect-[2/1] lg:aspect-[5/2] overflow-hidden rounded-2xl">
          {article.hero_image_url ? (
            <img
              src={article.hero_image_url}
              alt={article.title}
              className="w-full h-full object-cover group-hover:scale-105 transition-transform duration-700"
            />
          ) : (
            <div className="w-full h-full bg-neutral-200" />
          )}
          <div className="absolute inset-0 bg-gradient-to-t from-black/80 via-black/30 to-transparent" />
          <div className="absolute bottom-0 left-0 right-0 p-6 sm:p-10 lg:p-12">
            {article.category && (
              <span
                className="category-pill mb-4 bg-white/90 backdrop-blur-sm"
                style={{ color: article.category.color || '#1f4579', backgroundColor: `${article.category.color}20` }}
              >
                {article.category.name}
              </span>
            )}
            <h1 className="text-2xl sm:text-3xl lg:text-5xl font-serif font-bold text-white leading-tight max-w-4xl group-hover:text-primary-100 transition-colors">
              {article.title}
            </h1>
            {article.excerpt && (
              <p className="text-sm sm:text-lg text-white/80 mt-3 max-w-2xl leading-relaxed line-clamp-2">
                {article.excerpt}
              </p>
            )}
            <div className="flex items-center gap-3 mt-4 text-white/70 text-sm">
              {article.author && (
                <span className="font-medium text-white/90">{article.author.name}</span>
              )}
              <span>·</span>
              <span>{formatDate(article.published_at)}</span>
              <span>·</span>
              <span>{readingTime(article.body)}</span>
            </div>
          </div>
        </div>
      </Link>
    </article>
  )
}
