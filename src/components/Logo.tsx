function LogoMark({ className = 'h-10 w-10' }: { className?: string }) {
  return (
    <svg className={className} viewBox="0 0 66 61.29" fill="none" xmlns="http://www.w3.org/2000/svg">
      <polygon points="18.98 32.91 24.13 35.88 24.12 41.73 17.44 45.57 12.33 42.57 12.3 36.74 18.98 32.91" fill="#00ae9f" />
      <polygon points="24.52 19.81 24.52 26.02 19.21 29.06 12.27 25 12.32 18.83 17.58 15.77 24.52 19.81" fill="#00ae9f" />
      <polygon points="38.01 18.09 32.65 21.21 27.35 18.16 27.36 10.11 32.73 7.05 38.02 10.06 38.01 18.09" fill="#00ae9f" />
      <polygon points="46.2 28.97 40.84 25.84 40.89 19.73 47.89 15.77 53.2 18.93 53.19 25.01 46.2 28.97" fill="#00ae9f" />
      <polygon points="40.88 41.48 40.92 35.27 46.24 32.27 53.16 36.37 53.07 42.54 47.79 45.57 40.88 41.48" fill="#00ae9f" />
      <polygon points="27.35 43.16 32.72 40.03 38.01 43.08 38.01 51.12 32.65 54.19 27.36 51.19 27.35 43.16" fill="#00ae9f" />
      <polygon points="38.02 27.55 38.01 33.75 32.68 36.82 27.35 33.74 27.36 27.54 32.69 24.47 38.02 27.55" fill="#1f4579" />
      <polygon points="7.86 21.62 7.84 27.09 14.01 30.63 7.78 34.21 7.81 39.61 0 35.03 1.86 30.64 0 26.24 7.86 21.62" fill="#1f4579" />
      <polygon points="57.8 21.68 65.61 26.26 63.75 30.65 65.61 35.05 57.75 39.67 57.77 34.2 51.6 30.66 57.83 27.08 57.8 21.68" fill="#1f4579" />
      <polygon points="20.27 0 28.21 4.51 23.46 7.22 23.45 14.46 17.13 10.81 12.48 13.55 12.5 4.42 17.38 3.82 20.27 0" fill="#1f4579" />
      <polygon points="48.48 50.48 53.13 47.74 53.1 56.87 48.23 57.47 45.34 61.29 37.4 56.78 42.15 54.06 42.16 46.84 48.48 50.48" fill="#1f4579" />
      <polygon points="45.35 0.03 48.22 3.81 53.12 4.41 53.16 13.63 48.44 10.87 42.19 14.46 42.19 7.18 37.5 4.51 45.35 0.03" fill="#1f4579" />
      <polygon points="23.42 46.83 23.42 54.11 28.11 56.78 20.26 61.26 17.39 57.48 12.49 56.88 12.45 47.66 17.17 50.42 23.42 46.83" fill="#1f4579" />
    </svg>
  )
}

export default function Logo({ className = '' }: { className?: string }) {
  return (
    <div className={`flex items-center gap-2.5 ${className}`}>
      <LogoMark className="h-12 w-auto flex-shrink-0" />
      <div className="leading-none">
        <div className="font-sans font-bold text-ink-800 text-base uppercase tracking-wider">
          Informed Comment
        </div>
        <div className="text-[11px] text-neutral-500 font-sans mt-1 tracking-normal">
          Thoughts on the Middle East, History and Religion
        </div>
      </div>
    </div>
  )
}

export { LogoMark }
