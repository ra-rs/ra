#[doc = "Register `KRF` reader"]
pub struct R(crate::R<KRF_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<KRF_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<KRF_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<KRF_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `KRF` writer"]
pub struct W(crate::W<KRF_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<KRF_SPEC>;
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
impl From<crate::W<KRF_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<KRF_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `KIF0` reader - Key Interrupt Flag n"]
pub type KIF0_R = crate::BitReader<KIF0_A>;
#[doc = "Key Interrupt Flag n\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum KIF0_A {
    #[doc = "0: No interrupt detected"]
    _0 = 0,
    #[doc = "1: Interrupt detected"]
    _1 = 1,
}
impl From<KIF0_A> for bool {
    #[inline(always)]
    fn from(variant: KIF0_A) -> Self {
        variant as u8 != 0
    }
}
impl KIF0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> KIF0_A {
        match self.bits {
            false => KIF0_A::_0,
            true => KIF0_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == KIF0_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == KIF0_A::_1
    }
}
#[doc = "Field `KIF0` writer - Key Interrupt Flag n"]
pub type KIF0_W<'a, const O: u8> = crate::BitWriter<'a, u8, KRF_SPEC, KIF0_A, O>;
impl<'a, const O: u8> KIF0_W<'a, O> {
    #[doc = "No interrupt detected"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(KIF0_A::_0)
    }
    #[doc = "Interrupt detected"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(KIF0_A::_1)
    }
}
#[doc = "Field `KIF1` reader - Key Interrupt Flag n"]
pub type KIF1_R = crate::BitReader<KIF1_A>;
#[doc = "Key Interrupt Flag n\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum KIF1_A {
    #[doc = "0: No interrupt detected"]
    _0 = 0,
    #[doc = "1: Interrupt detected"]
    _1 = 1,
}
impl From<KIF1_A> for bool {
    #[inline(always)]
    fn from(variant: KIF1_A) -> Self {
        variant as u8 != 0
    }
}
impl KIF1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> KIF1_A {
        match self.bits {
            false => KIF1_A::_0,
            true => KIF1_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == KIF1_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == KIF1_A::_1
    }
}
#[doc = "Field `KIF1` writer - Key Interrupt Flag n"]
pub type KIF1_W<'a, const O: u8> = crate::BitWriter<'a, u8, KRF_SPEC, KIF1_A, O>;
impl<'a, const O: u8> KIF1_W<'a, O> {
    #[doc = "No interrupt detected"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(KIF1_A::_0)
    }
    #[doc = "Interrupt detected"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(KIF1_A::_1)
    }
}
#[doc = "Field `KIF2` reader - Key Interrupt Flag n"]
pub type KIF2_R = crate::BitReader<KIF2_A>;
#[doc = "Key Interrupt Flag n\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum KIF2_A {
    #[doc = "0: No interrupt detected"]
    _0 = 0,
    #[doc = "1: Interrupt detected"]
    _1 = 1,
}
impl From<KIF2_A> for bool {
    #[inline(always)]
    fn from(variant: KIF2_A) -> Self {
        variant as u8 != 0
    }
}
impl KIF2_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> KIF2_A {
        match self.bits {
            false => KIF2_A::_0,
            true => KIF2_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == KIF2_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == KIF2_A::_1
    }
}
#[doc = "Field `KIF2` writer - Key Interrupt Flag n"]
pub type KIF2_W<'a, const O: u8> = crate::BitWriter<'a, u8, KRF_SPEC, KIF2_A, O>;
impl<'a, const O: u8> KIF2_W<'a, O> {
    #[doc = "No interrupt detected"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(KIF2_A::_0)
    }
    #[doc = "Interrupt detected"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(KIF2_A::_1)
    }
}
#[doc = "Field `KIF3` reader - Key Interrupt Flag n"]
pub type KIF3_R = crate::BitReader<KIF3_A>;
#[doc = "Key Interrupt Flag n\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum KIF3_A {
    #[doc = "0: No interrupt detected"]
    _0 = 0,
    #[doc = "1: Interrupt detected"]
    _1 = 1,
}
impl From<KIF3_A> for bool {
    #[inline(always)]
    fn from(variant: KIF3_A) -> Self {
        variant as u8 != 0
    }
}
impl KIF3_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> KIF3_A {
        match self.bits {
            false => KIF3_A::_0,
            true => KIF3_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == KIF3_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == KIF3_A::_1
    }
}
#[doc = "Field `KIF3` writer - Key Interrupt Flag n"]
pub type KIF3_W<'a, const O: u8> = crate::BitWriter<'a, u8, KRF_SPEC, KIF3_A, O>;
impl<'a, const O: u8> KIF3_W<'a, O> {
    #[doc = "No interrupt detected"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(KIF3_A::_0)
    }
    #[doc = "Interrupt detected"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(KIF3_A::_1)
    }
}
impl R {
    #[doc = "Bit 0 - Key Interrupt Flag n"]
    #[inline(always)]
    pub fn kif0(&self) -> KIF0_R {
        KIF0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Key Interrupt Flag n"]
    #[inline(always)]
    pub fn kif1(&self) -> KIF1_R {
        KIF1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Key Interrupt Flag n"]
    #[inline(always)]
    pub fn kif2(&self) -> KIF2_R {
        KIF2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Key Interrupt Flag n"]
    #[inline(always)]
    pub fn kif3(&self) -> KIF3_R {
        KIF3_R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Key Interrupt Flag n"]
    #[inline(always)]
    #[must_use]
    pub fn kif0(&mut self) -> KIF0_W<0> {
        KIF0_W::new(self)
    }
    #[doc = "Bit 1 - Key Interrupt Flag n"]
    #[inline(always)]
    #[must_use]
    pub fn kif1(&mut self) -> KIF1_W<1> {
        KIF1_W::new(self)
    }
    #[doc = "Bit 2 - Key Interrupt Flag n"]
    #[inline(always)]
    #[must_use]
    pub fn kif2(&mut self) -> KIF2_W<2> {
        KIF2_W::new(self)
    }
    #[doc = "Bit 3 - Key Interrupt Flag n"]
    #[inline(always)]
    #[must_use]
    pub fn kif3(&mut self) -> KIF3_W<3> {
        KIF3_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Key Return Flag Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [krf](index.html) module"]
pub struct KRF_SPEC;
impl crate::RegisterSpec for KRF_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [krf::R](R) reader structure"]
impl crate::Readable for KRF_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [krf::W](W) writer structure"]
impl crate::Writable for KRF_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets KRF to value 0"]
impl crate::Resettable for KRF_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
