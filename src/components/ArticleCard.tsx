import { Link } from 'react-router-dom'
import type { Article } from '../types'
import { formatDateShort } from '../lib/utils'

export function categoryColor(article: Article): string {
  if (!article.category?.color) return '#1f4579'
  return article.category.color
}

export function categoryBgClass(article: Article): string {
  return 'bg-primary-100 text-primary-800'
}

export function categoryStyle(article: Article): React.CSSProperties {
  return {
    backgroundColor: article.category?.color ? `${article.category.color}20` : '#dde8f5',
    color: article.category?.color || '#1f4579',
  }
}

export default function ArticleCard({ article, variant = 'default' }: { article: Article; variant?: 'default' | 'compact' | 'horizontal' }) {
  if (variant === 'compact') {
    return (
      <Link to={`/article/${article.slug}`} className="group block">
        <div className="flex gap-3">
          {article.hero_image_url && (
            <img
              src={article.hero_image_url}
              alt={article.title}
              className="w-20 h-20 object-cover rounded-md flex-shrink-0"
              loading="lazy"
            />
          )}
          <div className="min-w-0">
            {article.category && (
              <span className="text-[10px] font-sans font-semibold uppercase tracking-wider" style={{ color: article.category.color || '#1f4579' }}>
                {article.category.name}
              </span>
            )}
            <h4 className="text-sm font-serif font-bold text-neutral-800 group-hover:text-primary-700 transition-colors leading-snug line-clamp-2 mt-0.5">
              {article.title}
            </h4>
            <span className="text-xs text-neutral-400 mt-1 block">{formatDateShort(article.published_at)}</span>
          </div>
        </div>
      </Link>
    )
  }

  if (variant === 'horizontal') {
    return (
      <Link to={`/article/${article.slug}`} className="group block">
        <div className="flex flex-col sm:flex-row gap-4">
          {article.hero_image_url && (
            <img
              src={article.hero_image_url}
              alt={article.title}
              className="w-full sm:w-64 h-48 sm:h-40 object-cover rounded-lg flex-shrink-0"
              loading="lazy"
            />
          )}
          <div className="flex-1">
            {article.category && (
              <span
                className="category-pill mb-2"
                style={categoryStyle(article)}
              >
                {article.category.name}
              </span>
            )}
            <h3 className="text-xl font-serif font-bold text-neutral-900 group-hover:text-primary-700 transition-colors leading-tight mt-2">
              {article.title}
            </h3>
            {article.excerpt && (
              <p className="text-sm text-neutral-600 mt-2 line-clamp-2 leading-relaxed">{article.excerpt}</p>
            )}
            <div className="flex items-center gap-2 mt-3 text-xs text-neutral-400">
              {article.author && (
                <span className="font-medium text-neutral-500">{article.author.name}</span>
              )}
              <span>·</span>
              <span>{formatDateShort(article.published_at)}</span>
            </div>
          </div>
        </div>
      </Link>
    )
  }

  return (
    <Link to={`/article/${article.slug}`} className="group block">
      {article.hero_image_url && (
        <div className="aspect-[16/10] overflow-hidden rounded-lg mb-4">
          <img
            src={article.hero_image_url}
            alt={article.title}
            className="w-full h-full object-cover group-hover:scale-105 transition-transform duration-500"
            loading="lazy"
          />
        </div>
      )}
      {article.category && (
        <span
          className="category-pill mb-3"
          style={categoryStyle(article)}
        >
          {article.category.name}
        </span>
      )}
      <h3 className="text-lg font-serif font-bold text-neutral-900 group-hover:text-primary-700 transition-colors leading-tight">
        {article.title}
      </h3>
      {article.excerpt && (
        <p className="text-sm text-neutral-600 mt-2 line-clamp-2 leading-relaxed">{article.excerpt}</p>
      )}
      <div className="flex items-center gap-2 mt-3 text-xs text-neutral-400">
        {article.author && (
          <span className="font-medium text-neutral-500">{article.author.name}</span>
        )}
        <span>·</span>
        <span>{formatDateShort(article.published_at)}</span>
      </div>
    </Link>
  )
}
