import { supabase } from '../lib/supabase'
import type { Article, Category, Author } from '../types'

export async function fetchCategories(): Promise<Category[]> {
  const { data, error } = await supabase
    .from('categories')
    .select('*')
    .order('name')
  if (error) throw error
  return data as Category[]
}

export async function fetchFeaturedArticle(): Promise<Article | null> {
  const { data, error } = await supabase
    .from('articles')
    .select(`
      *,
      category:categories(*),
      author:authors(*)
    `)
    .eq('featured', true)
    .order('published_at', { ascending: false })
    .limit(1)
    .maybeSingle()
  if (error) throw error
  return data as unknown as Article | null
}

export async function fetchRecentArticles(limit: number = 12, excludeId?: string): Promise<Article[]> {
  let query = supabase
    .from('articles')
    .select(`
      *,
      category:categories(*),
      author:authors(*)
    `)
    .order('published_at', { ascending: false })
    .limit(limit)
  if (excludeId) {
    query = query.neq('id', excludeId)
  }
  const { data, error } = await query
  if (error) throw error
  return data as unknown as Article[]
}

export async function fetchArticleBySlug(slug: string): Promise<Article | null> {
  const { data, error } = await supabase
    .from('articles')
    .select(`
      *,
      category:categories(*),
      author:authors(*)
    `)
    .eq('slug', slug)
    .maybeSingle()
  if (error) throw error
  return data as unknown as Article | null
}

export async function fetchArticlesByCategory(categorySlug: string, limit: number = 20): Promise<Article[]> {
  const { data: catData } = await supabase
    .from('categories')
    .select('id')
    .eq('slug', categorySlug)
    .maybeSingle()
  if (!catData) return []

  const { data, error } = await supabase
    .from('articles')
    .select(`
      *,
      category:categories(*),
      author:authors(*)
    `)
    .eq('category_id', catData.id)
    .order('published_at', { ascending: false })
    .limit(limit)
  if (error) throw error
  return data as unknown as Article[]
}

export async function fetchArticlesGroupedByCategory(perCategory: number = 3, excludeId?: string): Promise<{ category: Category; articles: Article[] }[]> {
  const { data: categories, error: catError } = await supabase
    .from('categories')
    .select('*')
  if (catError) throw catError
  if (!categories || categories.length === 0) return []

  const groups: { category: Category; articles: Article[] }[] = []

  for (const cat of categories) {
    let query = supabase
      .from('articles')
      .select(`
        *,
        category:categories(*),
        author:authors(*)
      `)
      .eq('category_id', cat.id)
      .order('published_at', { ascending: false })
      .limit(perCategory)
    if (excludeId) {
      query = query.neq('id', excludeId)
    }
    const { data, error } = await query
    if (error) throw error
    if (data && data.length > 0) {
      groups.push({ category: cat, articles: data as unknown as Article[] })
    }
  }

  groups.sort((a, b) => {
    const aDate = a.articles[0]?.published_at || ''
    const bDate = b.articles[0]?.published_at || ''
    return bDate.localeCompare(aDate)
  })

  return groups
}

export async function fetchArticlesByAuthor(authorSlug: string, limit: number = 20): Promise<Article[]> {
  const { data: authData } = await supabase
    .from('authors')
    .select('id')
    .eq('slug', authorSlug)
    .maybeSingle()
  if (!authData) return []

  const { data, error } = await supabase
    .from('articles')
    .select(`
      *,
      category:categories(*),
      author:authors(*)
    `)
    .eq('author_id', authData.id)
    .order('published_at', { ascending: false })
    .limit(limit)
  if (error) throw error
  return data as unknown as Article[]
}

export async function fetchAuthorBySlug(slug: string): Promise<Author | null> {
  const { data, error } = await supabase
    .from('authors')
    .select('*')
    .eq('slug', slug)
    .maybeSingle()
  if (error) throw error
  return data as Author | null
}

export async function fetchCategoryBySlug(slug: string): Promise<Category | null> {
  const { data, error } = await supabase
    .from('categories')
    .select('*')
    .eq('slug', slug)
    .maybeSingle()
  if (error) throw error
  return data as Category | null
}

export async function subscribeNewsletter(email: string): Promise<{ error: string | null }> {
  const { error } = await supabase
    .from('newsletter_subscribers')
    .insert({ email })
  if (error) {
    if (error.code === '23505') return { error: 'You are already subscribed.' }
    return { error: error.message }
  }
  return { error: null }
}
