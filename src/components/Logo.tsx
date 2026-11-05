export default function Logo({ className = 'h-8 w-8' }: { className?: string }) {
  return (
    <svg className={className} viewBox="0 0 48 48" fill="none" xmlns="http://www.w3.org/2000/svg">
      <circle cx="24" cy="24" r="23" stroke="#38704f" strokeWidth="2" />
      <path
        d="M14 34V14h2.5v8.5L26 14h3l-8 7.5L30 34h-3l-7.5-10.5L16.5 27v7H14z"
        fill="#38704f"
      />
      <circle cx="34" cy="16" r="3" fill="#ee7617" />
    </svg>
  )
}
