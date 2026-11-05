import { Link } from 'react-router-dom'

export default function NotFoundPage() {
  return (
    <div className="max-w-3xl mx-auto px-4 py-20 text-center">
      <h1 className="text-6xl font-serif font-bold text-primary-700 mb-4">404</h1>
      <p className="text-xl text-neutral-600 mb-8">Page not found</p>
      <Link to="/" className="inline-block px-6 py-3 bg-primary-600 text-white rounded-lg hover:bg-primary-700 transition-colors">
        Back to Home
      </Link>
    </div>
  )
}
