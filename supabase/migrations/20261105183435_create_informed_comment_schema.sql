/*
# Create Informed Comment schema

1. New Tables
- `authors`: Contributors and writers for the publication. Stores name, bio, avatar URL, and social links.
- `categories`: Top-level content categories (e.g., Middle East, Climate, US Politics, Culture). Has slug for URL routing.
- `articles`: The core content table. Each article has a title, slug, excerpt, body (HTML), hero image, category, author, published date, featured flag, and view count.
- `newsletter_subscribers`: Email addresses for newsletter signup. Stores email, created_at, and active flag.

2. Security
- This is a public news site with no reader sign-in. All content is publicly readable.
- SELECT on articles, authors, categories is granted to anon + authenticated (public content).
- INSERT on newsletter_subscribers granted to anon + authenticated (public signup).
- RLS enabled on all tables. Articles/authors/categories use USING(true) for SELECT since content is intentionally public.
- Newsletter subscribers can INSERT (signup) but cannot SELECT/UPDATE/DELETE (privacy).
- No user_id columns — this is a single-tenant public publication, not a multi-user app.

3. Indexes
- articles.slug (unique) for fast lookups by URL slug
- articles.category_id for filtering by category
- articles.published_at for ordering by date
- articles.featured for filtering featured stories
- authors.slug (unique) for fast lookups by URL slug
- categories.slug (unique) for fast lookups by URL slug
- newsletter_subscribers.email (unique) to prevent duplicates
*/

CREATE TABLE IF NOT EXISTS authors (
  id uuid PRIMARY KEY DEFAULT gen_random_uuid(),
  name text NOT NULL,
  slug text NOT NULL UNIQUE,
  bio text,
  avatar_url text,
  bluesky_url text,
  facebook_url text,
  created_at timestamptz DEFAULT now()
);

ALTER TABLE authors ENABLE ROW LEVEL SECURITY;
GRANT SELECT ON authors TO anon, authenticated;
DROP POLICY IF EXISTS "public_read_authors" ON authors;
CREATE POLICY "public_read_authors" ON authors FOR SELECT
  TO anon, authenticated USING (true);

CREATE TABLE IF NOT EXISTS categories (
  id uuid PRIMARY KEY DEFAULT gen_random_uuid(),
  name text NOT NULL,
  slug text NOT NULL UNIQUE,
  description text,
  color text,
  created_at timestamptz DEFAULT now()
);

ALTER TABLE categories ENABLE ROW LEVEL SECURITY;
GRANT SELECT ON categories TO anon, authenticated;
DROP POLICY IF EXISTS "public_read_categories" ON categories;
CREATE POLICY "public_read_categories" ON categories FOR SELECT
  TO anon, authenticated USING (true);

CREATE TABLE IF NOT EXISTS articles (
  id uuid PRIMARY KEY DEFAULT gen_random_uuid(),
  title text NOT NULL,
  slug text NOT NULL UNIQUE,
  excerpt text,
  body text NOT NULL,
  hero_image_url text,
  hero_image_caption text,
  category_id uuid REFERENCES categories(id) ON DELETE SET NULL,
  author_id uuid REFERENCES authors(id) ON DELETE SET NULL,
  tags text[] DEFAULT '{}',
  featured boolean DEFAULT false,
  published_at timestamptz NOT NULL DEFAULT now(),
  view_count integer DEFAULT 0,
  created_at timestamptz DEFAULT now()
);

ALTER TABLE articles ENABLE ROW LEVEL SECURITY;
GRANT SELECT ON articles TO anon, authenticated;
DROP POLICY IF EXISTS "public_read_articles" ON articles;
CREATE POLICY "public_read_articles" ON articles FOR SELECT
  TO anon, authenticated USING (true);

CREATE INDEX IF NOT EXISTS idx_articles_slug ON articles (slug);
CREATE INDEX IF NOT EXISTS idx_articles_category ON articles (category_id);
CREATE INDEX IF NOT EXISTS idx_articles_published ON articles (published_at DESC);
CREATE INDEX IF NOT EXISTS idx_articles_featured ON articles (featured);

CREATE TABLE IF NOT EXISTS newsletter_subscribers (
  id uuid PRIMARY KEY DEFAULT gen_random_uuid(),
  email text NOT NULL UNIQUE,
  active boolean DEFAULT true,
  created_at timestamptz DEFAULT now()
);

ALTER TABLE newsletter_subscribers ENABLE ROW LEVEL SECURITY;
GRANT INSERT ON newsletter_subscribers TO anon, authenticated;
DROP POLICY IF EXISTS "public_insert_newsletter" ON newsletter_subscribers;
CREATE POLICY "public_insert_newsletter" ON newsletter_subscribers FOR INSERT
  TO anon, authenticated WITH CHECK (true);
