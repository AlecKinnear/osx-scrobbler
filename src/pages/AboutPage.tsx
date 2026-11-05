import NewsletterSignup from '../components/NewsletterSignup'

export default function AboutPage() {
  return (
    <div>
      <div className="bg-gradient-to-br from-primary-800 to-primary-900 py-20">
        <div className="max-w-4xl mx-auto px-4 sm:px-6 lg:px-8 text-center">
          <h1 className="text-4xl sm:text-5xl font-serif font-bold text-white mb-4">
            About Informed Comment
          </h1>
          <p className="text-xl text-primary-100 leading-relaxed max-w-2xl mx-auto">
            Shedding light on how war, climate change and globalization are shaping our world.
          </p>
        </div>
      </div>

      <div className="max-w-3xl mx-auto px-4 sm:px-6 lg:px-8 py-16">
        <div className="article-body">
          <p className="text-lg leading-relaxed text-neutral-700">
            Drawing on the insights of expert journalists, activists, and academics, we strive to publish
            deep geopolitical analysis that's readable for a general audience. And unlike most foreign
            policy-oriented publications, our editorial line isn't dictated by beltway think tanks or
            corporate boards.
          </p>
        </div>

        <div className="mt-12">
          <h2 className="text-2xl font-serif font-bold text-neutral-900 mb-6">Editorial Board</h2>
          <div className="space-y-6">
            <div className="p-6 bg-neutral-50 rounded-xl border border-neutral-100">
              <h3 className="text-lg font-serif font-bold text-neutral-800">Editor-in-Chief: Juan Cole</h3>
              <p className="text-sm text-neutral-600 mt-2 leading-relaxed">
                Juan is a Distinguished University Professor at the University of Michigan. He is the
                author of, among many other books, <em>Muhammad: Prophet of Peace amid the Clash of Empires</em>{' '}
                and <em>The Rubaiyat of Omar Khayyam</em>. Follow him on{' '}
                <a href="https://bsky.app/profile/jricole.bsky.social" target="_blank" rel="noopener noreferrer" className="text-primary-600 hover:text-primary-700 underline">
                  Bluesky
                </a>{' '}
                or the{' '}
                <a href="https://www.facebook.com/informedcomment" target="_blank" rel="noopener noreferrer" className="text-primary-600 hover:text-primary-700 underline">
                  Informed Comment Facebook Page
                </a>.
              </p>
            </div>

            <div className="p-6 bg-neutral-50 rounded-xl border border-neutral-100">
              <h3 className="text-lg font-serif font-bold text-neutral-800">Editorial Advisory Council</h3>
              <ul className="mt-2 space-y-1 text-sm text-neutral-600">
                <li>David Faris</li>
                <li>Carlyn Meyer</li>
              </ul>
            </div>
          </div>
        </div>

        <div className="mt-12 p-8 bg-neutral-900 rounded-2xl text-white text-center">
          <h3 className="text-xl font-serif font-bold mb-2">Join Our Newsletter</h3>
          <p className="text-sm text-neutral-300 mb-4">Sharp analysis delivered to your inbox every day.</p>
          <div className="max-w-md mx-auto">
            <NewsletterSignup dark />
          </div>
        </div>

        <div className="mt-12 p-8 bg-primary-50 rounded-2xl border border-primary-100">
          <h3 className="text-lg font-serif font-bold text-primary-900 mb-2">Submissions</h3>
          <p className="text-sm text-primary-800 leading-relaxed">
            We accept unsolicited submissions. If you have a piece of deep geopolitical analysis
            that's readable for a general audience, we'd love to hear from you.
          </p>
        </div>
      </div>
    </div>
  )
}
