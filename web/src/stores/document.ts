import { defineStore } from 'pinia'
import { ref } from 'vue'
import api from '@/api'
import type { Document, CreateFolderRequest, UpdateDocumentRequest } from '@/types'

export const useDocumentStore = defineStore('document', () => {
  const documents = ref<Document[]>([])
  const currentDocument = ref<Document | null>(null)
  const currentFolderId = ref<string | null>(null)
  const loading = ref(false)

  async function fetchDocuments(folderId?: string): Promise<Document[]> {
    loading.value = true
    try {
      const params: any = { limit: 100, offset: 0 }
      if (folderId) {
        params.folder_id = folderId
      }
      const response = await api.documents.list(params)
      documents.value = response
      currentFolderId.value = folderId || null
      return response
    } finally {
      loading.value = false
    }
  }

  async function getDocument(id: string): Promise<Document> {
    loading.value = true
    try {
      const response = await api.documents.get(id)
      currentDocument.value = response
      return response
    } finally {
      loading.value = false
    }
  }

  async function createFolder(data: CreateFolderRequest): Promise<Document> {
    const response = await api.folders.create(data)
    await fetchDocuments(currentFolderId.value || undefined)
    return response
  }

  async function uploadFile(formData: FormData): Promise<Document> {
    const response = await api.documents.upload(formData)
    await fetchDocuments(currentFolderId.value || undefined)
    return response
  }

  async function updateDocument(id: string, data: UpdateDocumentRequest): Promise<Document> {
    const response = await api.documents.update(id, data)
    await fetchDocuments(currentFolderId.value || undefined)
    return response
  }

  async function deleteDocument(id: string): Promise<void> {
    await api.documents.delete(id)
    await fetchDocuments(currentFolderId.value || undefined)
  }

  async function downloadDocument(id: string): Promise<string> {
    return await api.documents.download(id)
  }

  async function moveDocument(id: string, targetFolderId?: string): Promise<Document> {
    const response = await api.documents.move(id, { target_folder_id: targetFolderId })
    await fetchDocuments(currentFolderId.value || undefined)
    return response
  }

  return {
    documents,
    currentDocument,
    currentFolderId,
    loading,
    fetchDocuments,
    getDocument,
    createFolder,
    uploadFile,
    updateDocument,
    deleteDocument,
    downloadDocument,
    moveDocument
  }
})

