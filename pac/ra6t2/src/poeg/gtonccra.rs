#[doc = "Register `GTONCCRA` reader"]
pub struct R(crate::R<GTONCCRA_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GTONCCRA_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GTONCCRA_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GTONCCRA_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GTONCCRA` writer"]
pub struct W(crate::W<GTONCCRA_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GTONCCRA_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<GTONCCRA_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GTONCCRA_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `NE` reader - Direct Stopping Request Setting"]
pub type NE_R = crate::BitReader<NE_A>;
#[doc = "Direct Stopping Request Setting\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NE_A {
    #[doc = "0: The signal for detection is not set as a direct stopping request signal"]
    _0 = 0,
    #[doc = "1: The signal for detection is set as a direct stopping request signal"]
    _1 = 1,
}
impl From<NE_A> for bool {
    #[inline(always)]
    fn from(variant: NE_A) -> Self {
        variant as u8 != 0
    }
}
impl NE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> NE_A {
        match self.bits {
            false => NE_A::_0,
            true => NE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == NE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == NE_A::_1
    }
}
#[doc = "Field `NE` writer - Direct Stopping Request Setting"]
pub type NE_W<'a, const O: u8> = crate::BitWriter<'a, u16, GTONCCRA_SPEC, NE_A, O>;
impl<'a, const O: u8> NE_W<'a, O> {
    #[doc = "The signal for detection is not set as a direct stopping request signal"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(NE_A::_0)
    }
    #[doc = "The signal for detection is set as a direct stopping request signal"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(NE_A::_1)
    }
}
#[doc = "Field `NFS` reader - Direct Stopping Request Selection"]
pub type NFS_R = crate::FieldReader<u8, NFS_A>;
#[doc = "Direct Stopping Request Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum NFS_A {
    #[doc = "0: Comparator level detection 0"]
    _0X0 = 0,
    #[doc = "1: Comparator level detection 1"]
    _0X1 = 1,
    #[doc = "2: Comparator level detection 2"]
    _0X2 = 2,
    #[doc = "4: Comparator level detection 3"]
    _0X4 = 4,
    #[doc = "7: GTETRGn pin input level detection (n = A to D)"]
    _0X7 = 7,
}
impl From<NFS_A> for u8 {
    #[inline(always)]
    fn from(variant: NFS_A) -> Self {
        variant as _
    }
}
impl NFS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<NFS_A> {
        match self.bits {
            0 => Some(NFS_A::_0X0),
            1 => Some(NFS_A::_0X1),
            2 => Some(NFS_A::_0X2),
            4 => Some(NFS_A::_0X4),
            7 => Some(NFS_A::_0X7),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_0X0`"]
    #[inline(always)]
    pub fn is_0x0(&self) -> bool {
        *self == NFS_A::_0X0
    }
    #[doc = "Checks if the value of the field is `_0X1`"]
    #[inline(always)]
    pub fn is_0x1(&self) -> bool {
        *self == NFS_A::_0X1
    }
    #[doc = "Checks if the value of the field is `_0X2`"]
    #[inline(always)]
    pub fn is_0x2(&self) -> bool {
        *self == NFS_A::_0X2
    }
    #[doc = "Checks if the value of the field is `_0X4`"]
    #[inline(always)]
    pub fn is_0x4(&self) -> bool {
        *self == NFS_A::_0X4
    }
    #[doc = "Checks if the value of the field is `_0X7`"]
    #[inline(always)]
    pub fn is_0x7(&self) -> bool {
        *self == NFS_A::_0X7
    }
}
#[doc = "Field `NFS` writer - Direct Stopping Request Selection"]
pub type NFS_W<'a, const O: u8> = crate::FieldWriter<'a, u16, GTONCCRA_SPEC, u8, NFS_A, 4, O>;
impl<'a, const O: u8> NFS_W<'a, O> {
    #[doc = "Comparator level detection 0"]
    #[inline(always)]
    pub fn _0x0(self) -> &'a mut W {
        self.variant(NFS_A::_0X0)
    }
    #[doc = "Comparator level detection 1"]
    #[inline(always)]
    pub fn _0x1(self) -> &'a mut W {
        self.variant(NFS_A::_0X1)
    }
    #[doc = "Comparator level detection 2"]
    #[inline(always)]
    pub fn _0x2(self) -> &'a mut W {
        self.variant(NFS_A::_0X2)
    }
    #[doc = "Comparator level detection 3"]
    #[inline(always)]
    pub fn _0x4(self) -> &'a mut W {
        self.variant(NFS_A::_0X4)
    }
    #[doc = "GTETRGn pin input level detection (n = A to D)"]
    #[inline(always)]
    pub fn _0x7(self) -> &'a mut W {
        self.variant(NFS_A::_0X7)
    }
}
#[doc = "Field `NFV` reader - Direct Stopping Request Active Sense"]
pub type NFV_R = crate::BitReader<NFV_A>;
#[doc = "Direct Stopping Request Active Sense\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NFV_A {
    #[doc = "0: Stopping output is requested when the output stopping detection signal is 0"]
    _0 = 0,
    #[doc = "1: Stopping output is requested when the output stopping detection signal is 1"]
    _1 = 1,
}
impl From<NFV_A> for bool {
    #[inline(always)]
    fn from(variant: NFV_A) -> Self {
        variant as u8 != 0
    }
}
impl NFV_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> NFV_A {
        match self.bits {
            false => NFV_A::_0,
            true => NFV_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == NFV_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == NFV_A::_1
    }
}
#[doc = "Field `NFV` writer - Direct Stopping Request Active Sense"]
pub type NFV_W<'a, const O: u8> = crate::BitWriter<'a, u16, GTONCCRA_SPEC, NFV_A, O>;
impl<'a, const O: u8> NFV_W<'a, O> {
    #[doc = "Stopping output is requested when the output stopping detection signal is 0"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(NFV_A::_0)
    }
    #[doc = "Stopping output is requested when the output stopping detection signal is 1"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(NFV_A::_1)
    }
}
impl R {
    #[doc = "Bit 0 - Direct Stopping Request Setting"]
    #[inline(always)]
    pub fn ne(&self) -> NE_R {
        NE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 4:7 - Direct Stopping Request Selection"]
    #[inline(always)]
    pub fn nfs(&self) -> NFS_R {
        NFS_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bit 8 - Direct Stopping Request Active Sense"]
    #[inline(always)]
    pub fn nfv(&self) -> NFV_R {
        NFV_R::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Direct Stopping Request Setting"]
    #[inline(always)]
    #[must_use]
    pub fn ne(&mut self) -> NE_W<0> {
        NE_W::new(self)
    }
    #[doc = "Bits 4:7 - Direct Stopping Request Selection"]
    #[inline(always)]
    #[must_use]
    pub fn nfs(&mut self) -> NFS_W<4> {
        NFS_W::new(self)
    }
    #[doc = "Bit 8 - Direct Stopping Request Active Sense"]
    #[inline(always)]
    #[must_use]
    pub fn nfv(&mut self) -> NFV_W<8> {
        NFV_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "GPT Output Stopping Control Group A Controlling Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gtonccra](index.html) module"]
pub struct GTONCCRA_SPEC;
impl crate::RegisterSpec for GTONCCRA_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [gtonccra::R](R) reader structure"]
impl crate::Readable for GTONCCRA_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gtonccra::W](W) writer structure"]
impl crate::Writable for GTONCCRA_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets GTONCCRA to value 0x0100"]
impl crate::Resettable for GTONCCRA_SPEC {
    const RESET_VALUE: Self::Ux = 0x0100;
}
