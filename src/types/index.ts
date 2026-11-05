export interface Author {
  id: string
  name: string
  slug: string
  bio: string | null
  avatar_url: string | null
  bluesky_url: string | null
  facebook_url: string | null
  created_at: string
}

export interface Category {
  id: string
  name: string
  slug: string
  description: string | null
  color: string | null
  created_at: string
}

export interface Article {
  id: string
  title: string
  slug: string
  excerpt: string | null
  body: string
  hero_image_url: string | null
  hero_image_caption: string | null
  category_id: string | null
  author_id: string | null
  tags: string[]
  featured: boolean
  published_at: string
  view_count: number
  created_at: string
  category?: Category
  author?: Author
}

export interface ArticleWithRelations extends Article {
  category?: Category
  author?: Author
}
