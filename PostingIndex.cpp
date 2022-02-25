#include "PostingIndex.h"
#include "FileIndexer.h"

PostingIndex::PostingIndex() {}

bool PostingIndex::postFile(path_t path) {
    FileIndexer fIndexer(path);
    std::optional<file_record_map_t&> file_record_map_opt = fIndexer.indexFile();
    if(!file_record_map_opt.has_value()) return false;
    file_record_map_t& file_record_map = file_record_map_opt.value();

}