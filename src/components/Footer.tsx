import { Link } from 'react-router-dom'
import Logo from './Logo'
import NewsletterSignup from './NewsletterSignup'

export default function Footer() {
  return (
    <footer className="bg-ink-900 text-neutral-300 mt-20">
      <div className="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 py-16">
        <div className="grid grid-cols-1 md:grid-cols-3 gap-12">
          <div>
            <div className="mb-4 bg-white rounded-lg px-3 py-2 inline-block">
              <Logo />
            </div>
            <p className="text-sm text-neutral-400 leading-relaxed max-w-xs">
              Shedding light on how war, climate change and globalization are shaping our world.
              Deep geopolitical analysis from expert journalists, activists, and academics.
            </p>
          </div>

          <div>
            <h4 className="text-sm font-sans font-semibold text-white uppercase tracking-wider mb-4">Sections</h4>
            <ul className="space-y-2 text-sm">
              <li><Link to="/category/middle-east" className="hover:text-primary-300 transition-colors">Middle East</Link></li>
              <li><Link to="/category/us-politics" className="hover:text-primary-300 transition-colors">US Politics</Link></li>
              <li><Link to="/category/climate-crisis" className="hover:text-primary-300 transition-colors">Climate Crisis</Link></li>
              <li><Link to="/category/culture" className="hover:text-primary-300 transition-colors">Culture</Link></li>
              <li><Link to="/category/religion" className="hover:text-primary-300 transition-colors">Religion</Link></li>
            </ul>
          </div>

          <div>
            <h4 className="text-sm font-sans font-semibold text-white uppercase tracking-wider mb-4">Newsletter</h4>
            <p className="text-sm text-neutral-400 mb-4">Sharp analysis delivered to your inbox.</p>
            <NewsletterSignup dark />
          </div>
        </div>

        <div className="border-t border-neutral-700 mt-12 pt-8 flex flex-col sm:flex-row justify-between items-center gap-4">
          <p className="text-xs text-neutral-500">© {new Date().getFullYear()} Informed Comment. All Rights Reserved.</p>
          <div className="flex items-center gap-4 text-xs">
            <Link to="/about" className="text-neutral-400 hover:text-white transition-colors">About</Link>
            <a href="https://www.facebook.com/informedcomment" target="_blank" rel="noopener noreferrer" className="text-neutral-400 hover:text-white transition-colors">Facebook</a>
            <a href="https://bsky.app/profile/jricole.bsky.social" target="_blank" rel="noopener noreferrer" className="text-neutral-400 hover:text-white transition-colors">Bluesky</a>
          </div>
        </div>
      </div>
    </footer>
  )
}
