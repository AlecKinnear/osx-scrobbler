export default function LoadingState() {
  return (
    <div className="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 py-20">
      <div className="animate-pulse">
        <div className="h-8 w-64 bg-neutral-200 rounded mb-8" />
        <div className="grid grid-cols-1 lg:grid-cols-3 gap-8">
          <div className="lg:col-span-2 space-y-6">
            <div className="aspect-[2/1] bg-neutral-200 rounded-xl" />
            <div className="h-6 w-3/4 bg-neutral-200 rounded" />
            <div className="h-6 w-1/2 bg-neutral-200 rounded" />
            <div className="space-y-3">
              <div className="h-4 w-full bg-neutral-100 rounded" />
              <div className="h-4 w-full bg-neutral-100 rounded" />
              <div className="h-4 w-2/3 bg-neutral-100 rounded" />
            </div>
          </div>
          <div className="space-y-4">
            <div className="h-40 bg-neutral-100 rounded-xl" />
            <div className="h-40 bg-neutral-100 rounded-xl" />
          </div>
        </div>
      </div>
    </div>
  )
}
