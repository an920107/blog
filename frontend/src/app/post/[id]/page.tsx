import React from 'react'

type Props = {
  params: { id: string };
}

export default function PostContentPage(props: Props) {
  return (
    <>
      <div className="container mb-10 md:mb-24">
        <h2 className="my-8 md:my-14">{props.params.id}</h2>
      </div>
    </>
  )
}
