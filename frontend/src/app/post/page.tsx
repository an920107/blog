import { FontAwesomeIcon } from '@fortawesome/react-fontawesome';
import { faClock } from '@fortawesome/free-regular-svg-icons';
import { faHashtag, faMagnifyingGlass, faFilter } from '@fortawesome/free-solid-svg-icons';
import { Post } from '@/@types/post';
import { format } from 'date-fns';
import React from 'react';
import Link from 'next/link';

type Props = {}

export default function PostPage({ }: Props) {
  return (
    <>
      <div className="container">
        <div className="flex flex-row justify-between items-center">
          <h2 className="my-8 md:my-14">文章</h2>
          <p>
            <button className="mx-2 md:mx-4">
              <FontAwesomeIcon icon={faMagnifyingGlass} className="h-6 w-6 md:h-8 md:w-8" />
            </button>
            <button className="mx-2 md:mx-4">
              <FontAwesomeIcon icon={faFilter} className="h-6 w-6 md:h-8 md:w-8" />
            </button>
          </p>
        </div>
        <div className="grid grid-cols-1 md:grid-cols-2 xl:grid-cols-3 gap-x-8 gap-y-10">
          {
            posts.map((post) => (
              <Link key={post.id} href={`/post/${post.id}`} className="font-normal">
                <div className="rounded-xl shadow-md transition-shadow duration-300 hover:shadow-xl image-container">
                  <div className="relative">
                    <img src={post.previewImage} className="rounded-xl aspect-video object-cover" />
                    <div className="image-overlay rounded-xl"></div>
                  </div>
                  <div className="grid grid-cols-1 gap-y-2 p-4">
                    <h3>{post.title}</h3>
                    <p className="text-justify text-ellipsis line-clamp-2 h-12">{post.description}</p>
                    <p className="flex items-center">
                      <FontAwesomeIcon icon={faHashtag} className="h-4 w-4 me-2" />
                      {
                        post.tags.map((tag) => (
                          <span key={tag} className="bg-blue-100 px-2 me-1 rounded-full">{tag}</span>
                        ))
                      }
                    </p>
                    <p className="flex items-center">
                      <FontAwesomeIcon icon={faClock} className="h-4 w-4 me-3" />
                      {format(post.updatedTime, "yyyy-MM-dd")}
                    </p>
                  </div>
                </div>
              </Link>
            ))
          }
        </div>
      </div>
    </>
  )
}

const posts: Post[] = [
  {
    id: "1",
    title: "Hello World",
    description: "Hello World Hello World Hello World Hello World Hello World Hello World. Hello World Hello World Hello World Hello World Hello World Hello World.",
    tags: ["hello", "tech", "coding"],
    content: "Hello World Hello World Hello World Hello World.",
    previewImage: "https://www.alleycat.org/wp-content/uploads/2019/03/FELV-cat.jpg",
    createdTime: new Date(2024, 0, 1),
    updatedTime: new Date(2024, 0, 1),
  },
  {
    id: "2",
    title: "Hello World",
    description: "Hello World Hello World Hello World Hello World Hello World Hello World.",
    tags: ["hello", "tech", "coding"],
    content: "Hello World Hello World Hello World Hello World.",
    previewImage: "https://cdn.britannica.com/36/234736-050-4AC5B6D5/Scottish-fold-cat.jpg",
    createdTime: new Date(2024, 0, 1),
    updatedTime: new Date(2024, 0, 1),
  },
  {
    id: "3",
    title: "Hello World",
    description: "Hello World Hello World Hello.",
    tags: ["hello", "tech", "coding"],
    content: "Hello World Hello World Hello World Hello World.",
    previewImage: "https://i.natgeofe.com/n/9135ca87-0115-4a22-8caf-d1bdef97a814/75552.jpg",
    createdTime: new Date(2024, 0, 1),
    updatedTime: new Date(2024, 0, 1),
  },
  {
    id: "4",
    title: "Hello World",
    description: "Hello World Hello World Hello World Hello World Hello World Hello World.",
    tags: ["hello", "tech", "coding"],
    content: "Hello World Hello World Hello World Hello World.",
    previewImage: "https://static.scientificamerican.com/sciam/cache/file/F1E90F3D-1FFD-4BA9-9DFA98AE647FA7D4_source.jpg?w=1200",
    createdTime: new Date(2024, 0, 1),
    updatedTime: new Date(2024, 0, 1),
  },
  {
    id: "5",
    title: "Hello World",
    description: "Hello World Hello World Hello World Hello World Hello World Hello World.",
    tags: ["hello", "tech", "coding"],
    content: "Hello World Hello World Hello World Hello World.",
    previewImage: "https://static.scientificamerican.com/sciam/cache/file/2AE14CDD-1265-470C-9B15F49024186C10_source.jpg?w=1200",
    createdTime: new Date(2024, 0, 1),
    updatedTime: new Date(2024, 0, 1),
  },
  {
    id: "6",
    title: "Hello World",
    description: "Hello World Hello World Hello World Hello World Hello World Hello World.",
    tags: ["hello", "tech", "coding"],
    content: "Hello World Hello World Hello World Hello World.",
    previewImage: "https://upload.wikimedia.org/wikipedia/commons/thumb/1/15/Cat_August_2010-4.jpg/1200px-Cat_August_2010-4.jpg",
    createdTime: new Date(2024, 0, 1),
    updatedTime: new Date(2024, 0, 1),
  }
];