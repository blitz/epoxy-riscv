// The below constants and structures are from the VirtIO 1.1
// specification, which is distributed under the following license.

/*
  Copyright © OASIS Open 2018. All Rights Reserved.

  All capitalized terms in the following text have the meanings
  assigned to them in the OASIS Intellectual Property Rights Policy
  (the "OASIS IPR Policy"). The full Policy may be found at the OASIS
  website.

  This document and translations of it may be copied and furnished to
  others, and derivative works that comment on or otherwise explain it
  or assist in its implementation may be prepared, copied, published,
  and distributed, in whole or in part, without restriction of any
  kind, provided that the above copyright notice and this section are
  included on all such copies and derivative works. However, this
  document itself may not be modified in any way, including by
  removing the copyright notice or references to OASIS, except as
  needed for the purpose of developing any document or deliverable
  produced by an OASIS Technical Committee (in which case the rules
  applicable to copyrights, as set forth in the OASIS IPR Policy, must
  be followed) or as required to translate it into languages other
  than English.

  This specification is provided under the Non-Assertion Mode of the
  OASIS IPR Policy, the mode chosen when the Technical Committee was
  established. For information on whether any patents have been
  disclosed that may be essential to implementing this specification,
  and any offers of patent licensing terms, please refer to the
  Intellectual Property Rights section of the TC’s web page
  (https://github.com/oasis-tcs/virtio-admin/blob/master/IPR.md).

  The limited permissions granted above are perpetual and will not be
  revoked by OASIS or its successors or assigns.

  This document and the information contained herein is provided on an
  "AS IS" basis and OASIS DISCLAIMS ALL WARRANTIES, EXPRESS OR
  IMPLIED, INCLUDING BUT NOT LIMITED TO ANY WARRANTY THAT THE USE OF
  THE INFORMATION HEREIN WILL NOT INFRINGE ANY OWNERSHIP RIGHTS OR ANY
  IMPLIED WARRANTIES OF MERCHANTABILITY OR FITNESS FOR A PARTICULAR
  PURPOSE.

  OASIS requests that any OASIS Party or any other party that believes
  it has patent claims that would necessarily be infringed by
  implementations of this OASIS Committee Specification or OASIS
  Standard, to notify OASIS TC Administrator and provide an indication
  of its willingness to grant patent licenses to such patent claims in
  a manner consistent with the IPR Mode of the OASIS Technical
  Committee that produced this specification.

  OASIS invites any party to contact the OASIS TC Administrator if it
  is aware of a claim of ownership of any patent claims that would
  necessarily be infringed by implementations of this specification by
  a patent holder that is not willing to provide a license to such
  patent claims in a manner consistent with the IPR Mode of the OASIS
  Technical Committee that produced this specification. OASIS may
  include such claims on its website, but disclaims any obligation to
  do so.

  OASIS takes no position regarding the validity or scope of any
  intellectual property or other rights that might be claimed to
  pertain to the implementation or use of the technology described in
  this document or the extent to which any license under such rights
  might or might not be available; neither does it represent that it
  has made any effort to identify any such rights. Information on
  OASIS’ procedures with respect to rights in any document or
  deliverable produced by an OASIS Technical Committee can be found on
  the OASIS website. Copies of claims of rights made available for
  publication and any assurances of licenses to be made available, or
  the result of an attempt made to obtain a general license or
  permission for the use of such proprietary rights by implementers or
  users of this OASIS Committee Specification or OASIS Standard, can
  be obtained from the OASIS TC Administrator. OASIS makes no
  representation that any information or list of intellectual property
  rights will at any time be complete, or that any claims in such list
  are, in fact, Essential Claims.

  The name "OASIS" is a trademark of OASIS, the owner and developer of
  this specification, and should be used only to refer to the
  organization and its official outputs. OASIS welcomes reference to,
  and implementation and use of, specifications, while reserving the
  right to enforce its marks against misleading uses. Please see
  https://www.oasis-open.org/policies-guidelines/trademark for above
  guidance.
 */

#pragma once

#include <types.hpp>

namespace virtio
{
using le64 = uint64_t;
using le32 = uint32_t;
using le16 = uint16_t;
using u8 = uint8_t;

enum pci_vendor_cap_type : u8 {
  /* Common configuration */
  PCI_CAP_COMMON_CFG = 1,
  /* Notifications */
  PCI_CAP_NOTIFY_CFG = 2,
  /* ISR Status */
  PCI_CAP_ISR_CFG = 3,
  /* Device specific configuration */
  PCI_CAP_DEVICE_CFG = 4,
  /* PCI configuration access */
  PCI_CAP_PCI_CFG = 5,
};

enum status : u8 {
  ACKNOWLEDGE = 1,
  DRIVER = 2,
  FAILED = 128,
  FEATURES_OK = 8,
  DRIVER_OK = 4,
  DEVICE_NEEDS_RESET = 64,
};

enum features : int {
  VIRTIO_F_RING_INDIRECT_DESC = 28,
  VIRTIO_F_RING_EVENT_IDX = 29,
  VIRTIO_F_VERSION_1 = 32,
  VIRTIO_F_ACCESS_PLATFORM = 33,
  VIRTIO_F_RING_PACKED = 34,
  VIRTIO_F_IN_ORDER = 35,
  VIRTIO_F_ORDER_PLATFORM = 36,
  VIRTIO_F_SR_IOV = 37,
  VIRTIO_F_NOTIFICATION_DATA = 38,
};

struct pci_common_cfg {
  /* About the whole device. */
  le32 device_feature_select;
  le32 const device_feature;
  le32 driver_feature_select;
  le32 driver_feature;
  le16 msix_config;
  le16 const num_queues;
  u8 device_status;
  u8 const config_generation;

  /* About a specific virtqueue. */
  le16 queue_select;
  le16 queue_size;
  le16 queue_msix_vector;
  le16 queue_enable;
  le16 const queue_notify_off;
  le64 queue_desc;
  le64 queue_driver;
  le64 queue_device;
};

struct virtio_net_config {
  u8 mac[6];
  le16 status;
  le16 max_virtqueue_pairs;
  le16 mtu;
};

}  // namespace virtio
