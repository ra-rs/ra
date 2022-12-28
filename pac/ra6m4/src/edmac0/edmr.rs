#[doc = "Register `EDMR` reader"]
pub struct R(crate::R<EDMR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EDMR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EDMR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EDMR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `EDMR` writer"]
pub struct W(crate::W<EDMR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EDMR_SPEC>;
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
impl From<crate::W<EDMR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EDMR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SWR` reader - Software Reset"]
pub type SWR_R = crate::BitReader<bool>;
#[doc = "Field `SWR` writer - Software Reset"]
pub type SWR_W<'a, const O: u8> = crate::BitWriter<'a, u32, EDMR_SPEC, bool, O>;
#[doc = "Field `DL` reader - Transmit/Receive Descriptor Length"]
pub type DL_R = crate::FieldReader<u8, DL_A>;
#[doc = "Transmit/Receive Descriptor Length\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum DL_A {
    #[doc = "0: 16 bytes"]
    _00 = 0,
    #[doc = "1: 32 bytes"]
    _01 = 1,
    #[doc = "2: 64 bytes"]
    _10 = 2,
    #[doc = "3: 16 bytes."]
    _11 = 3,
}
impl From<DL_A> for u8 {
    #[inline(always)]
    fn from(variant: DL_A) -> Self {
        variant as _
    }
}
impl DL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DL_A {
        match self.bits {
            0 => DL_A::_00,
            1 => DL_A::_01,
            2 => DL_A::_10,
            3 => DL_A::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == DL_A::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == DL_A::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == DL_A::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == DL_A::_11
    }
}
#[doc = "Field `DL` writer - Transmit/Receive Descriptor Length"]
pub type DL_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, EDMR_SPEC, u8, DL_A, 2, O>;
impl<'a, const O: u8> DL_W<'a, O> {
    #[doc = "16 bytes"]
    #[inline(always)]
    pub fn _00(self) -> &'a mut W {
        self.variant(DL_A::_00)
    }
    #[doc = "32 bytes"]
    #[inline(always)]
    pub fn _01(self) -> &'a mut W {
        self.variant(DL_A::_01)
    }
    #[doc = "64 bytes"]
    #[inline(always)]
    pub fn _10(self) -> &'a mut W {
        self.variant(DL_A::_10)
    }
    #[doc = "16 bytes."]
    #[inline(always)]
    pub fn _11(self) -> &'a mut W {
        self.variant(DL_A::_11)
    }
}
#[doc = "Field `DE` reader - Big Endian Mode/Little Endian Mode"]
pub type DE_R = crate::BitReader<DE_A>;
#[doc = "Big Endian Mode/Little Endian Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DE_A {
    #[doc = "0: Big endian mode"]
    _0 = 0,
    #[doc = "1: Little endian mode."]
    _1 = 1,
}
impl From<DE_A> for bool {
    #[inline(always)]
    fn from(variant: DE_A) -> Self {
        variant as u8 != 0
    }
}
impl DE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DE_A {
        match self.bits {
            false => DE_A::_0,
            true => DE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DE_A::_1
    }
}
#[doc = "Field `DE` writer - Big Endian Mode/Little Endian Mode"]
pub type DE_W<'a, const O: u8> = crate::BitWriter<'a, u32, EDMR_SPEC, DE_A, O>;
impl<'a, const O: u8> DE_W<'a, O> {
    #[doc = "Big endian mode"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DE_A::_0)
    }
    #[doc = "Little endian mode."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DE_A::_1)
    }
}
impl R {
    #[doc = "Bit 0 - Software Reset"]
    #[inline(always)]
    pub fn swr(&self) -> SWR_R {
        SWR_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 4:5 - Transmit/Receive Descriptor Length"]
    #[inline(always)]
    pub fn dl(&self) -> DL_R {
        DL_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bit 6 - Big Endian Mode/Little Endian Mode"]
    #[inline(always)]
    pub fn de(&self) -> DE_R {
        DE_R::new(((self.bits >> 6) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Software Reset"]
    #[inline(always)]
    #[must_use]
    pub fn swr(&mut self) -> SWR_W<0> {
        SWR_W::new(self)
    }
    #[doc = "Bits 4:5 - Transmit/Receive Descriptor Length"]
    #[inline(always)]
    #[must_use]
    pub fn dl(&mut self) -> DL_W<4> {
        DL_W::new(self)
    }
    #[doc = "Bit 6 - Big Endian Mode/Little Endian Mode"]
    #[inline(always)]
    #[must_use]
    pub fn de(&mut self) -> DE_W<6> {
        DE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "EDMAC Mode Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [edmr](index.html) module"]
pub struct EDMR_SPEC;
impl crate::RegisterSpec for EDMR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [edmr::R](R) reader structure"]
impl crate::Readable for EDMR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [edmr::W](W) writer structure"]
impl crate::Writable for EDMR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets EDMR to value 0"]
impl crate::Resettable for EDMR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
